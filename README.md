# LibraAI Protocol

Decentralized Artificial Intelligence for the Solana ecosystem.  
Run inference on-chain. Spawn agents. Own your thoughts.  
Powered by $LIBRAI.

## 🔮 What is LibraAI?

LibraAI is an open protocol that enables decentralized AI agents to be spawned, configured, and operated fully on-chain.  
Each agent is an autonomous intelligence with customizable behavior, decision-making logic, and data access rules.

Forget OpenAI.  
This is open thought.

## ✨ Features

- 🧠 On-chain agent initialization via Anchor
- 📡 Modular AI behavior encoded in Solana programs
- 📁 Distributed config (IPFS-compatible)
- 💸 Tokenized access and staking via $LIBRAI
- 🔓 100% open-source / experimental / ungoverned

## 📦 Quick Start

```bash
git clone https://github.com/librai-protocol/librai-protocol
cd librai-protocol
yarn install
solana-test-validator
```

```bash
anchor build
anchor test
anchor run spawn-agent
```

## 🧪 Smart Contract

See programs/librai/src/lib.rs

## 🪙 $LIBRAI Token

- Used to spawn & stake agents
- Required for write-access to global AI memory pool
- Burned when agents are destroyed
- Supply: dynamic via protocol demand
