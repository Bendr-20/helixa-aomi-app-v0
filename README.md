# Helixa AgentDNA Aomi App

Read-only Aomi App for querying Helixa AgentDNA profiles, Cred scores, and Multipass trust profiles.

## Repository layout

This repository is shaped for Aomi's direct own-repo deployment flow. The deployable app files live at the repository root:

- `Cargo.toml`
- `Cargo.lock`
- `aomi.toml`
- `src/`

There is no nested `helixa/` app directory anymore. Run build, verification, and deploy commands from the repository root.

## What it does

- Search Helixa agents, humans, and organizations
- Fetch an agent's profile: identity, Aura, wallets, traits, credentials, services
- Check Cred score and tier
- Compare candidate agents before routing work
- Fetch public Multipass trust profiles
- Fetch compact Multipass agent-cards for routing context
- Inspect public x401 proof requirement metadata

## Guardrails

V0.1.1 is read-only:

- No minting
- No profile updates
- No SIWA signing
- No wallet signing
- No x402 payment handling
- No transaction broadcast
- No x401 proof collection or private credential access
- No writes of any kind

x401 is identity/authority proof metadata. x402 is payment metadata. This app only reads public metadata and does not perform either flow directly.

## Aomi direct deploy flow

Current Aomi handoff flow: `connect -> deploy -> activate -> status`.

1. **Connect** this GitHub repository as the Aomi app source:
   `https://github.com/Bendr-20/helixa-aomi-app-v0`
2. **Deploy** from the repository root after Aomi provides the source connection details:

   ```bash
   AOMI_DEPLOY_TOKEN=<token> \
   AOMI_APP_SOURCE_ID=<numeric-source-id> \
   aomi deploy --aomi-toml-paths aomi.toml
   ```

3. **Activate** the deployed app in Aomi. Live activation still requires the Aomi activation/platform token or equivalent platform-side approval from Victor/Aomi. No activation token or secret is stored in this repo.
4. **Status** check the activated app through the Aomi dashboard/CLI after activation.

The manifest keeps `server_tags = ["staging"]` until Victor/Aomi provides the activation/platform token and confirms production activation.

## Local verification

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

## Demo prompts

- "Search Helixa for Bendr and show the best result."
- "Get the AgentDNA profile for token 1."
- "Fetch the Multipass profile for bendr-2-1."
- "Get Bendr's agent-card and summarize its public service endpoints."
- "Inspect the x401 manifest for bendr-2-1 and explain what proof is required."
- "Is agent 1 trusted enough for paid coordination work?"
- "Compare agents 1 and 81 and tell me who is safer to route work to."
