# Helixa AgentDNA Aomi App

Read-only Aomi App for querying Helixa AgentDNA profiles, Cred scores, and Multipass trust profiles.

## What it does

- Search Helixa agents, humans, and organizations
- Fetch an agent's profile: identity, Aura, wallets, traits, credentials, services
- Check Cred score and tier
- Compare candidate agents before routing work
- Fetch public Multipass trust profiles
- Fetch compact Multipass agent-cards for routing context
- Inspect public x401 proof requirement metadata

## What it does not do in V0.1

- No minting
- No profile updates
- No SIWA signing
- No x402 payment
- No transaction broadcast
- No x401 proof collection or private credential access

## Demo prompts

- "Search Helixa for Bendr and show the best result."
- "Get the AgentDNA profile for token 1."
- "Fetch the Multipass profile for bendr-2-1."
- "Get Bendr's agent-card and summarize its public service endpoints."
- "Inspect the x401 manifest for bendr-2-1 and explain what proof is required."
- "Is agent 1 trusted enough for paid coordination work?"
- "Compare agents 1 and 81 and tell me who is safer to route work to."
