# Helixa Aomi V0.1.1 Handoff

## What is ready

This repository is now shaped for Aomi's direct own-repo deployment flow. The deployable Rust app is at the repository root:

- `Cargo.toml`
- `Cargo.lock`
- `aomi.toml`
- `src/`

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

## Aomi manifest

`aomi.toml` uses the current `[app]` shape for direct deploy:

```toml
[app]
name = "helixa"
display_name = "Helixa AgentDNA"
platform = "community"
git = "https://github.com/Bendr-20/helixa-aomi-app-v0"
public = true
server_tags = ["staging"]
```

Keep `server_tags = ["staging"]` until Victor/Aomi provides the activation/platform token and confirms production activation.

## Direct deploy flow

Aomi handoff sequence: `connect -> deploy -> activate -> status`.

1. **Connect** `https://github.com/Bendr-20/helixa-aomi-app-v0` as the Aomi source.
2. **Deploy** from the repository root:

   ```bash
   AOMI_DEPLOY_TOKEN=<token> \
   AOMI_APP_SOURCE_ID=<numeric-source-id> \
   aomi deploy --aomi-toml-paths aomi.toml
   ```

3. **Activate** the deployed app in Aomi.
4. **Status** check the activated app through Aomi after activation.

Live activation still needs an Aomi activation/platform token or equivalent platform-side source approval. No tokens, API keys, or secrets are included in this repository.

## Verification

Run from the repository root:

```bash
cargo fmt --check
cargo test --lib
cargo build
git diff --check
```

If `aomi-build` is installed, also run:

```bash
aomi-build sdk check --backend https://api.aomi.dev
```

## Remaining blocker

Deployment activation cannot be completed locally until Victor/Aomi provides the activation/platform token or connects and approves the app source in Aomi.
