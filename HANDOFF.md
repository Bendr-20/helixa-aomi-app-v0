# Helixa Aomi V0.1.1 Handoff

## What is ready

The `helixa/` crate is the handoff package for Aomi/community-apps review.

It exposes seven read-only tools:

- `search_agents`
- `get_agent_profile`
- `check_cred`
- `compare_agents`
- `get_multipass_profile`
- `get_agent_card`
- `get_x401_manifest`

V0.1.1 adds Multipass public trust profile context, compact agent-card lookup, and public x401 manifest lookup.

## Guardrails

- Read-only only.
- No SIWA signing.
- No x402 payment handling.
- No x401 proof collection or private credential access.
- No wallet signing, minting, profile updates, transaction broadcast, or writes.
- x401 is identity/authority proof metadata. x402 is payment metadata. This app only reads public metadata.

## Aomi deploy requirements

Aomi deploy cannot be completed from this machine yet because the required deployment credentials are not configured:

- `AOMI_DEPLOY_TOKEN` or `--activation-token`
- `AOMI_APP_SOURCE_ID` or `--app-source-id` as a positive integer
- A non-default app API key may also be required to test `--app helixa` after activation.

The app manifest is in the current documented `[app]` shape and pins `aomi-sdk = "=3.0.0"`, matching the community platform `required_sdk_version`.

## Suggested redeploy command

Run from the `helixa/` directory after the source repo is connected in Aomi:

```bash
AOMI_DEPLOY_TOKEN=<token> \
AOMI_APP_SOURCE_ID=<numeric-source-id> \
aomi deploy --dry-run --aomi-toml-paths aomi.toml

AOMI_DEPLOY_TOKEN=<token> \
AOMI_APP_SOURCE_ID=<numeric-source-id> \
aomi deploy --aomi-toml-paths aomi.toml
```

Then activate/test in Aomi with the app key if required.

## Local verification

Fresh verification commands used for this package:

```bash
cargo fmt --check
cargo test --lib
cargo build
```
