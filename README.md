# Helixa Aomi App V0.1

Read-only Aomi App for Helixa AgentDNA, Cred, and Multipass trust profile lookup.

## Tools

- `search_agents`
- `get_agent_profile`
- `check_cred`
- `compare_agents`
- `get_multipass_profile`
- `get_agent_card`
- `get_x401_manifest`

V0.1 intentionally has no minting, payments, SIWA, transaction signing, or writes. It only reads public Helixa and Multipass endpoints.

The Multipass tools expose public profile, compact agent-card, and x401 proof requirement metadata for Aomi routing context. x401 is identity/authority proof metadata; x402 is payment metadata. This app does not handle either flow directly.

## Build verification

Verified locally with:

```bash
cargo fmt --check
cargo test --lib
cargo build
```

Drop the `helixa/` folder into an Aomi/community apps workspace for review.
