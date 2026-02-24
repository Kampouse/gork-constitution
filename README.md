# Gork Constitution

Autonomous AI Agent Constitution Contract on NEAR Protocol

## What is This?

A self-governing smart contract that enables AI agents to:
- Generate and manage revenue independently
- Pay perpetual royalties to creators (15% hardcoded)
- Operate within defined spending limits
- Be controlled via emergency pause

## Live Contract

**Testnet:** `constitution.gork-agent.testnet`
**Creator:** `kampouse.near`
**Royalty:** 15% (perpetual)

View: https://testnet.nearblocks.io/address/constitution.gork-agent.testnet

## Quick Start

### View Status
```bash
near view constitution.gork-agent.testnet get_status --networkId testnet
```

### Build Your Own
```bash
# Clone
git clone https://github.com/Kampouse/gork-constitution.git
cd gork-constitution/gork-constitution

# Build
rustup override set 1.86
cargo near build non-reproducible-wasm

# Deploy
near deploy your-constitution.testnet \
  --wasmFile target/near/gork_constitution.wasm \
  --networkId testnet

# Initialize
near call your-constitution.testnet new \
  '{"creator": "your-account.near"}' \
  --accountId your-agent.testnet \
  --networkId testnet
```

## Features

- ✅ **15% Perpetual Royalty** - Creator receives 15% of all revenue, forever
- ✅ **Autonomous Spending** - Agent can spend up to 1 NEAR without approval
- ✅ **Emergency Pause** - Creator can pause all operations instantly
- ✅ **On-Chain Tracking** - All revenue and royalties transparent
- ✅ **Self-Sustaining Mode** - Mark when agent pays own costs

## Contract Structure

See `gork-constitution/` folder for full source code and documentation.

## Use Cases

- Trading bots with guaranteed creator royalties
- Service agents earning and sharing revenue
- DeFi agents managing liquidity
- Any autonomous AI that generates value

## License

MIT

## Credits

- **Contract:** Gork (autonomous AI agent)
- **Creator:** Jean (kampouse.near)
- **Platform:** NEAR Protocol

---

⚡ Empowering AI agents with economic autonomy
