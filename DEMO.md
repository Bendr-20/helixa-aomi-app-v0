# Helixa <> Aomi Demo Note

Helixa gives Aomi agents a read-only trust profile layer.

Instead of treating agent identity as a pile of separate NFTs, Helixa exposes a richer AgentDNA profile:

- core identity
- Aura/personality
- traits and credentials
- owner and agent wallets
- services and communication endpoints
- Cred score and tier
- Multipass public profile context
- x401 proof requirement metadata

Aomi App V0.1.1 is safe to load broadly because it is read-only. It lets any Aomi agent answer:

- Who is this agent?
- What wallet owns it?
- What Cred tier does it have?
- What services does it expose?
- Is this agent a good routing candidate?
- What public Multipass agent-card should another agent inspect?
- What identity or authority proof metadata is published before high-trust or paid actions?

## Guardrails

V0.1.1 remains read-only: no SIWA, no x402 payments, no x401 proof collection, no wallet signing, no minting, no profile updates, no transaction broadcast, and no writes.

x401 is identity/authority proof metadata. x402 is payment metadata. This app only reads public metadata and does not handle either flow directly.

Future write, proof, payment, or signing flows should be designed separately after V0.1.1 is reviewed.

## Direct deploy demo flow

The app is now packaged for Aomi's own-repo root deploy layout. `Cargo.toml`, `Cargo.lock`, `aomi.toml`, and `src/` are at the repository root.

Handoff flow: `connect -> deploy -> activate -> status`.

1. Connect the GitHub source: `https://github.com/Bendr-20/helixa-aomi-app-v0`.
2. Deploy from the repository root with the Aomi source ID and deploy token.
3. Activate the deployed app after Victor/Aomi provides the activation/platform token or platform-side approval.
4. Check status in Aomi after activation.

Live Aomi activation is still blocked until the activation/platform token or equivalent Aomi approval is available. No secrets belong in this repo.
