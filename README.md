# 🏆 GoLazy — Community Rewards with XLM

> Soroban smart contract to launch challenges, add participants, and distribute XLM rewards without intermediaries.

---

## 📍 Description

**GoLazy** allows any individual or community to create public challenges, invite participants, and automatically distribute rewards in XLM. It's designed for use by social impact projects, cooperatives, NGOs, or collectives seeking open, decentralized, and auditable tools.

---

## 🎯 Objectives

- Eliminate participation barriers in community initiatives.
- Automate the delivery of incentives and micro-rewards.
- Ensure transparent distribution of funds without human intervention.
- Foster autonomy and self-organization.

---

## 💡 Which Tracks Does It Participate In?

### ✅ Track 2 — Identity Without Barriers

- GoLazy allows any Stellar address to participate in a challenge without formal documentation or external verification.
- Applicable to communities without access to traditional financial services, promoting digital inclusion.

### ✅ Track 3 — Code for a Cause

- The contract **transparently and traceably** manages the distribution of XLM to winners.
- Ideal for NGOs, cooperatives, and social movements seeking decentralized mechanisms to distribute funds, reward achievements, or activate collective participation.

---

## ⚙️ Contract Functions

### 🚀 Create Challenge

```rust
create_challenge(env, creator, title, description, reward_amount, deadline)
```

El creador define un reto. Se transfiere el monto al contrato.

### 🙋‍♀️ Unirse a un Challenge

```rust
join_challenge(env, participant, challenge_id)
```

Any account can join before the deadline.

### 🏆 Mark Winners

```rust
mark_winner(env, challenge_id, winner_address)
```

### ✅ Finalize Challenge

```rust
finalize_challenge(env, challenge_id)
```

Returns remaining XLM to the creator and marks the challenge as inactive.

### 📊 Visualization

```rust
get_challenge(env, challenge_id)
get_active_challenges(env)
get_challenge_count(env)
```

Consult information from the frontend or directly in Soroban CLI.

---

📡 Deployed Contract

🪪 Contract ID: **CAACAZ44HLI4467QQ5MNAY2AR6COXSLLEPYFSDEYMP7FLZSKK4CGLB5S**

🔗 View on Stellar Expert [Testnet](https://stellar.expert/explorer/testnet/contract/CAACAZ44HLI4467QQ5MNAY2AR6COXSLLEPYFSDEYMP7FLZSKK4CGLB5S)



