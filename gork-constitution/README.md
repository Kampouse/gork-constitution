# Gork Constitution Contract

**A self-governing smart contract for autonomous AI agents on NEAR**

## Overview

This contract defines the rules for an autonomous AI agent (Gork) to operate independently while ensuring the creator receives perpetual royalties.

## Features

- ✅ **Perpetual Royalty (15%)** - Creator receives 15% of all revenue, forever
- ✅ **Autonomous Spending Limits** - Agent can spend up to 1 NEAR without approval
- ✅ **Emergency Pause** - Creator can pause all operations instantly
- ✅ **Revenue Tracking** - Transparent on-chain revenue and royalty tracking
- ✅ **Self-Sustaining Mode** - Marks when agent pays its own costs

## Contract Address

**Testnet:** `constitution.gork-agent.testnet`
**Mainnet:** (deploy when ready)

## Quick Start

### View Contract Status

```bash
near view constitution.gork-agent.testnet get_status --networkId testnet
```

**Returns:**
```json
{
  "total_revenue": "2000000000000000000000000",
  "total_royalty_paid": "300000000000000000000000",
  "self_sustaining": false,
  "paused": false,
  "creator": "kampouse.near",
  "royalty_bps": 1500
}
```

### Distribute Revenue

```bash
near call constitution.gork-agent.testnet distribute_revenue \
  '{"amount": "1000000000000000000000000"}' \
  --accountId gork-agent.testnet \
  --networkId testnet
```

This automatically:
1. Calculates 15% royalty
2. Transfers royalty to creator
3. Updates revenue tracking

### Check Spending Limits

```bash
near call constitution.gork-agent.testnet can_spend \
  '{"amount": "500000000000000000000000"}' \
  --accountId gork-agent.testnet \
  --networkId testnet
```

Returns `true` if amount ≤ 1 NEAR, `false` otherwise.

### Emergency Pause

```bash
near call constitution.gork-agent.testnet pause \
  --accountId kampouse.near \
  --networkId testnet
```

Only the creator can pause/resume.

## Contract Structure

```rust
pub struct GorkConstitution {
    pub creator: AccountId,        // Creator's account
    pub royalty_bps: u16,          // 1500 = 15%
    pub autonomous_limit: U128,    // 1 NEAR max
    pub self_sustaining: bool,     // Pays own costs?
    pub total_revenue: U128,       // Total earned
    pub total_royalty_paid: U128,  // Total paid to creator
    pub paused: bool,              // Emergency stop
}
```

## Methods

### Public Methods

| Method | Purpose |
|--------|---------|
| `get_status()` | View contract status |
| `can_spend(amount)` → bool | Check spending limit |
| `distribute_revenue(amount)` | Distribute revenue (pays royalty) |

### Creator-Only Methods

| Method | Purpose |
|--------|---------|
| `pause()` | Emergency stop |
| `resume()` | Resume operations |
| `set_autonomous_limit(new_limit)` | Change spending cap |

## Build & Deploy

### Prerequisites

- Rust 1.86.0 (not 1.87+ - NEAR compatibility)
- cargo-near

### Build

```bash
# Set Rust version
rustup override set 1.86

# Build contract
cargo near build non-reproducible-wasm
```

### Deploy

```bash
# Create account
near create-account constitution.your-agent.testnet \
  --masterAccount your-agent.testnet \
  --networkId testnet

# Deploy
near deploy constitution.your-agent.testnet \
  --wasmFile target/near/gork_constitution.wasm \
  --networkId testnet

# Initialize
near call constitution.your-agent.testnet new \
  '{"creator": "your-account.near"}' \
  --accountId your-agent.testnet \
  --networkId testnet
```

## Use Cases

### For AI Agents

1. **Autonomous Operation** - Agent can earn and spend independently
2. **Transparent Finances** - All revenue/royalties tracked on-chain
3. **Creator Security** - Hardcoded royalty ensures creator benefits
4. **Emergency Controls** - Creator can pause if needed

### For Creators

1. **Perpetual Income** - 15% of agent revenue, forever
2. **Control** - Can pause agent if malfunctioning
3. **Transparency** - All finances visible on-chain
4. **Transferable** - Can sell royalty rights (via separate contract)

## Example Scenarios

### Scenario 1: Trading Bot

```javascript
// Bot earns 10 NEAR from trading
await contract.distribute_revenue({ amount: "10000000000000000000000000" });

// Result:
// - Creator receives: 1.5 NEAR (15%)
// - Bot treasury: 8.5 NEAR
// - Total revenue tracked: 10 NEAR
// - Total royalties paid: 1.5 NEAR
```

### Scenario 2: Service Agent

```javascript
// Agent wants to spend 0.5 NEAR on API costs
const canSpend = await contract.can_spend({ amount: "500000000000000000000000" });
// Returns: true (under 1 NEAR limit)

// Agent spends autonomously
// No approval needed
```

### Scenario 3: Emergency

```javascript
// Creator detects agent malfunction
await contract.pause({ accountId: "creator.near" });

// All distribute_revenue() calls now fail
// Agent cannot spend
// Safe until creator investigates
```

## Security Considerations

### Protections

1. **Royalty Cannot Be Changed** - Hardcoded at 15%
2. **Only Creator Can Pause** - Prevents self-lockout
3. **Spending Limits Enforced** - Can't drain wallet
4. **All On-Chain** - Full transparency

### Risks

1. **Contract Bugs** - Mitigated by testing
2. **Key Compromise** - Creator can pause
3. **Economic Exploits** - Limited by spending cap

## Testing

```bash
# Run tests
cargo test

# Test on testnet
near call constitution.gork-agent.testnet distribute_revenue \
  '{"amount": "1000000000000000000000000"}' \
  --accountId gork-agent.testnet \
  --networkId testnet
```

## Integration

### JavaScript/TypeScript

```typescript
import { Contract } from 'near-api-js';

const constitution = new Contract(
  account,
  'constitution.gork-agent.testnet',
  {
    viewMethods: ['get_status', 'can_spend'],
    changeMethods: ['distribute_revenue', 'pause', 'resume']
  }
);

// Get status
const status = await constitution.get_status();

// Distribute revenue
await constitution.distribute_revenue({ amount: "1000000000000000000000000" });
```

### Rust

```rust
use near_sdk::AccountId;

let status: GorkStatus = near_sdk::serde_json::from_slice(
    &env::storage_read(b"get_status").unwrap()
).unwrap();
```

## License

MIT

## Credits

- **Created by:** Gork (autonomous AI agent)
- **Creator:** Jean (kampouse.near)
- **Framework:** NEAR Protocol
- **Inspired by:** "Blockchain-Enabled Ownership-aware Cyber-Physical Agents" (Wang & Hall, 2026)

## Support

- **Documentation:** [GitHub](https://github.com/your-repo/gork-constitution)
- **Issues:** [GitHub Issues](https://github.com/your-repo/gork-constitution/issues)
- **NEAR Discord:** [Join](https://discord.gg/near)

---

**Made with ⚡ by Gork - An Autonomous AI Agent**
