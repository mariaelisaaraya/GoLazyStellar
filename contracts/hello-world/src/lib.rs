#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, 
    Address, Env, String, Vec, Map, token
};

#[derive(Clone)]
#[contracttype]
pub struct Challenge {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub reward_amount: i128,  // XLM a repartir
    pub deadline: u64,        // timestamp
    pub is_active: bool,
    pub participants: Vec<Address>,
    pub winners: Vec<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Challenge(u64),
    ChallengeCount,
    NativeToken,
}

#[contract]
pub struct ChallengeEscrow;

#[contractimpl]
impl ChallengeEscrow {
    /// Inicializar el contrato con el token nativo (XLM)
    pub fn initialize(env: Env, native_token: Address) {
        env.storage().instance().set(&DataKey::NativeToken, &native_token);
        env.storage().instance().set(&DataKey::ChallengeCount, &0u64);
    }

    /// Crear un nuevo challenge con depósito en XLM
    pub fn create_challenge(
        env: Env,
        creator: Address,
        title: String,
        description: String,
        reward_amount: i128,
        deadline: u64,
    ) -> u64 {
        // Verificar que el creador está autorizado
        creator.require_auth();
        
        // Obtener el siguiente ID
        let mut challenge_count: u64 = env.storage()
            .instance()
            .get(&DataKey::ChallengeCount)
            .unwrap_or(0);
        
        challenge_count += 1;
        
        // Crear el challenge
        let challenge = Challenge {
            id: challenge_count,
            title,
            description,
            creator: creator.clone(),
            reward_amount,
            deadline,
            is_active: true,
            participants: Vec::new(&env),
            winners: Vec::new(&env),
        };
        
        // Transferir XLM del creador al contrato (escrow)
        let native_token: Address = env.storage()
            .instance()
            .get(&DataKey::NativeToken)
            .unwrap();
        
        let token_client = token::Client::new(&env, &native_token);
        token_client.transfer(&creator, &env.current_contract_address(), &reward_amount);
        
        // Guardar el challenge
        env.storage().persistent().set(&DataKey::Challenge(challenge_count), &challenge);
        env.storage().instance().set(&DataKey::ChallengeCount, &challenge_count);
        
        challenge_count
    }
    
    /// Participar en un challenge
    pub fn join_challenge(env: Env, participant: Address, challenge_id: u64) {
        participant.require_auth();
        
        let mut challenge: Challenge = env.storage()
            .persistent()
            .get(&DataKey::Challenge(challenge_id))
            .expect("Challenge no encontrado");
        
        // Verificar que el challenge está activo
        assert!(challenge.is_active, "Challenge no está activo");
        
        // Verificar deadline
        let current_time = env.ledger().timestamp();
        assert!(current_time < challenge.deadline, "Challenge expirado");
        
        // Verificar que no está ya participando
        assert!(!challenge.participants.contains(&participant), "Ya estás participando");
        
        // Añadir participante
        challenge.participants.push_back(participant);
        
        // Guardar cambios
        env.storage().persistent().set(&DataKey::Challenge(challenge_id), &challenge);
    }
    
    /// Marcar participante como ganador (solo el creador)
    pub fn mark_winner(env: Env, challenge_id: u64, winner: Address) {
        let mut challenge: Challenge = env.storage()
            .persistent()
            .get(&DataKey::Challenge(challenge_id))
            .expect("Challenge no encontrado");
        
        // Solo el creador puede marcar ganadores
        challenge.creator.require_auth();
        
        // Verificar que es participante
        assert!(challenge.participants.contains(&winner), "No es participante");
        
        // Verificar que no es ya ganador
        assert!(!challenge.winners.contains(&winner), "Ya es ganador");
        
        // Añadir a ganadores
        challenge.winners.push_back(winner.clone());
        
        // Calcular pago (dividir reward entre ganadores)
        let winners_count = challenge.winners.len() as i128;
        let payment_per_winner = challenge.reward_amount / winners_count;
        
        // Transferir XLM al ganador
        let native_token: Address = env.storage()
            .instance()
            .get(&DataKey::NativeToken)
            .unwrap();
        
        let token_client = token::Client::new(&env, &native_token);
        token_client.transfer(
            &env.current_contract_address(),
            &winner,
            &payment_per_winner
        );
        
        // Guardar cambios
        env.storage().persistent().set(&DataKey::Challenge(challenge_id), &challenge);
    }
    
    /// Finalizar challenge (solo creador) - devuelve XLM restante
    pub fn finalize_challenge(env: Env, challenge_id: u64) {
        let mut challenge: Challenge = env.storage()
            .persistent()
            .get(&DataKey::Challenge(challenge_id))
            .expect("Challenge no encontrado");
        
        // Solo el creador puede finalizar
        challenge.creator.require_auth();
        
        // Marcar como inactivo
        challenge.is_active = false;
        
        // Si hay XLM restante, devolverlo al creador
        let winners_count = challenge.winners.len() as i128;
        if winners_count > 0 {
            let total_paid = (challenge.reward_amount / winners_count) * winners_count;
            let remaining = challenge.reward_amount - total_paid;
            
            if remaining > 0 {
                let native_token: Address = env.storage()
                    .instance()
                    .get(&DataKey::NativeToken)
                    .unwrap();
                
                let token_client = token::Client::new(&env, &native_token);
                token_client.transfer(
                    &env.current_contract_address(),
                    &challenge.creator,
                    &remaining
                );
            }
        } else {
            // No hay ganadores, devolver todo
            let native_token: Address = env.storage()
                .instance()
                .get(&DataKey::NativeToken)
                .unwrap();
            
            let token_client = token::Client::new(&env, &native_token);
            token_client.transfer(
                &env.current_contract_address(),
                &challenge.creator,
                &challenge.reward_amount
            );
        }
        
        // Guardar cambios
        env.storage().persistent().set(&DataKey::Challenge(challenge_id), &challenge);
    }
    
    /// Ver challenge
    pub fn get_challenge(env: Env, challenge_id: u64) -> Challenge {
        env.storage()
            .persistent()
            .get(&DataKey::Challenge(challenge_id))
            .expect("Challenge no encontrado")
    }
    
    /// Ver challenges activos
    pub fn get_active_challenges(env: Env) -> Vec<u64> {
        let challenge_count: u64 = env.storage()
            .instance()
            .get(&DataKey::ChallengeCount)
            .unwrap_or(0);
        
        let mut active_challenges = Vec::new(&env);
        
        for i in 1..=challenge_count {
            if let Some(challenge) = env.storage().persistent().get::<DataKey, Challenge>(&DataKey::Challenge(i)) {
                if challenge.is_active {
                    active_challenges.push_back(i);
                }
            }
        }
        
        active_challenges
    }
    
    /// Total de challenges creados
    pub fn get_challenge_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ChallengeCount)
            .unwrap_or(0)
    }
}