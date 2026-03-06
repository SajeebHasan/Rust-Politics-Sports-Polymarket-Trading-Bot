# Polymarket Sports & Politics Trailing Bot

A Rust bot for [Polymarket](https://polymarket.com) that trades **sports and politics (binary) markets** by slug using a **trailing stop** strategy only.

**Supported markets:** Any binary market on Polymarket—sports (e.g. game outcomes), politics (e.g. election results), or other event markets. You provide the market **slug**; the bot does the rest.

**Behavior:**
- You set a **slug** in config (e.g. the market’s event slug).
- The bot loads that single market and tracks both outcome tokens.
- It trails the token whose price is moving down first: when price recovers (current ask ≥ lowest seen + trailing stop), it buys that token.
- After the first buy, it trails the **opposite** token the same way and buys when that side triggers.
- You can run **once** (one pair of buys per market) or **continuous** (after both sides are bought, it resets and trails/buys again until the market ends).

---

## Quick start

| Binary | Description |
|--------|-------------|
| `main_sports_trailing` | Sports & politics trailing bot (default) — slug-based, trailing only |

```bash
# Build
cargo build --release

# Simulation (no real orders)
cargo run --release -- --simulation

# Live
cargo run --release -- --no-simulation
```

---

## Setup

1. **Install Rust** (if needed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Build:**
   ```bash
   cargo build --release
   ```

3. **Configure:** Create or edit `config.json` with:
   - **polymarket:** `gamma_api_url`, `clob_api_url`, `api_key`, `api_secret`, `api_passphrase`, `private_key`
   - Optional: `proxy_wallet_address`, `signature_type` (1 = POLY_PROXY, 2 = GNOSIS_SAFE)
   - **trading:**  
     - **`slug`** (required) — market slug, e.g. `"nfl-team-a-vs-team-b"` (sports) or `"will-x-win-election"` (politics)  
     - **`continuous`** — `true` = keep trailing and buying both sides repeatedly until market ends; `false` = buy each side once per market  
     - `trailing_stop_point` (e.g. `0.03`)  
     - `trailing_shares` (e.g. `10`)  
     - `check_interval_ms` (e.g. `1000`)

---

## Configuration

- **`--simulation`** / **`--no-simulation`** — No real orders in simulation.
- **`--config <path>`** — Config file (default: `config.json`).

**Relevant config fields:**
- **polymarket:** `gamma_api_url`, `clob_api_url`, `api_key`, `api_secret`, `api_passphrase`, `private_key`, optional `proxy_wallet_address`, `signature_type`.
- **trading:** `slug` (required), `continuous`, `trailing_stop_point`, `trailing_shares`, `check_interval_ms`.

---

## Notes

- The bot runs until the market ends (by end time) or you stop it (Ctrl+C).
- Simulation mode logs trades but does not place orders.

---

## Security

- Do **not** commit `config.json` with real keys or secrets.
- Prefer simulation and small sizes when testing.
- Monitor logs and balances when running in production.
