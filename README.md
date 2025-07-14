# 🏆 GoLazy — Recompensas Comunitarias con XLM

> Contrato inteligente Soroban para lanzar desafíos, sumar participantes y distribuir recompensas en XLM sin intermediarios.

---

## 📍 Descripción

**GoLazy** permite a cualquier persona o comunidad crear retos públicos, invitar participantes y distribuir recompensas automáticamente en XLM. Está diseñado para ser usado por proyectos de impacto social, cooperativas, ONGs o colectivos que buscan herramientas abiertas, descentralizadas y auditables.

---

## 🎯 Objetivos

- Eliminar barreras de participación en iniciativas comunitarias.
- Automatizar la entrega de incentivos y microrecompensas.
- Transparentar la distribución de fondos sin intervención humana.
- Fomentar la autonomía y la auto-organización.

---

## 💡 ¿En qué tracks participa?

### ✅ Track 2 — Identidad sin barreras

- GoLazy permite que cualquier dirección en Stellar pueda participar de un reto sin documentación formal o verificación externa.
- Aplicable a comunidades sin acceso a servicios financieros tradicionales, fomentando la inclusión digital.

### ✅ Track 3 — Código para la causa

- El contrato gestiona de forma **transparente y trazable** la distribución de XLM a ganadores.
- Ideal para ONGs, cooperativas y movimientos sociales que buscan mecanismos descentralizados para distribuir fondos, recompensar logros o activar participación colectiva.

---

## ⚙️ Funciones del Contrato

### 🚀 Crear Challenge

```rust
create_challenge(env, creator, title, description, reward_amount, deadline)
```

El creador define un reto. Se transfiere el monto al contrato.

### 🙋‍♀️ Unirse a un Challenge

```rust
join_challenge(env, participant, challenge_id)
```

Cualquier cuenta puede sumarse antes del deadline.

### 🏆 Marcar Ganadores

```rust
mark_winner(env, challenge_id, winner_address)
```

### ✅ Finalizar Challenge

```rust
finalize_challenge(env, challenge_id)
```

Devuelve XLM sobrante al creador y marca el reto como inactivo.

### 📊 Visualización

```rust
get_challenge(env, challenge_id)
get_active_challenges(env)
get_challenge_count(env)
```

Consultar información desde el frontend o directamente en Soroban CLI.

---

📡 Contrato Desplegado

🪪 ID del contrato: CAACAZ44HLI4467QQ5MNAY2AR6COXSLLEPYFSDEYMP7FLZSKK4CGLB5S

🔗 Ver en Stellar Expert [Testnet](https://stellar.expert/explorer/testnet/contract/CAACAZ44HLI4467QQ5MNAY2AR6COXSLLEPYFSDEYMP7FLZSKK4CGLB5S)



