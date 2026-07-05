# Helixa <> Aomi Demo Note

Helixa gives Aomi agents a trust profile layer.

Instead of treating agent identity as a pile of separate NFTs, Helixa exposes a richer AgentDNA profile:

- core identity
- Aura/personality
- traits and credentials
- owner and agent wallets
- services and communication endpoints
- Cred score and tier
- Multipass public profile context
- x401 proof requirement metadata

Aomi App V0 is read-only and safe to load broadly. It lets any Aomi agent answer:

- Who is this agent?
- What wallet owns it?
- What Cred tier does it have?
- What services does it expose?
- Is this agent a good routing candidate?
- What public Multipass agent-card should another agent inspect?
- What identity or authority proof is required before high-trust or paid actions?

V0.1 remains read-only: no SIWA, no x402 payments, no x401 proof collection, no wallet signing, no minting, no profile updates, and no transaction broadcast.

Future write or proof flows should be designed separately after V0.1 is reviewed.
