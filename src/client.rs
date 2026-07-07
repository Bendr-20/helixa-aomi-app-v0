use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

const API_BASE_URL: &str = "https://api.helixa.xyz";
const MULTIPASS_BASE_URL: &str = "https://helixa.xyz";

#[derive(Clone, Debug, Default)]
pub(crate) struct HelixaApp;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchAgentsArgs {
    /// Search query: agent name, wallet address, human name, organization, skill, or keyword.
    pub query: String,
    /// Maximum results to return. Defaults to 5 and clamps to 20.
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAgentProfileArgs {
    /// Helixa agent token ID on Base.
    pub token_id: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CheckCredArgs {
    /// Helixa agent token ID on Base.
    pub token_id: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CompareAgentsArgs {
    /// Two to five Helixa agent token IDs to compare.
    pub token_ids: Vec<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMultipassProfileArgs {
    /// Public Multipass ID, slug, or resolvable identifier, such as bendr-2-1 or mp_helixa_agent_1.
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAgentCardArgs {
    /// Public Multipass ID, slug, or resolvable identifier, such as bendr-2-1 or mp_helixa_agent_1.
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetX401ManifestArgs {
    /// Public Multipass ID, slug, or resolvable identifier, such as bendr-2-1 or mp_helixa_agent_1.
    pub id: String,
}

pub(crate) struct HelixaClient {
    http: reqwest::blocking::Client,
}

impl HelixaClient {
    pub(crate) fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(12))
            .user_agent("aomi-helixa/0.1.1")
            .build()
            .map_err(|err| format!("failed to create Helixa HTTP client: {err}"))?;
        Ok(Self { http })
    }

    pub(crate) fn search_path(query: &str, limit: u32) -> String {
        format!("/api/v2/search?q={}&limit={}", url_component(query), limit)
    }

    pub(crate) fn agent_path(token_id: u64) -> String {
        format!("/api/v2/agent/{token_id}")
    }

    pub(crate) fn cred_path(token_id: u64) -> String {
        format!("/api/v2/agent/{token_id}/cred")
    }

    pub(crate) fn multipass_profile_path(id: &str) -> String {
        format!("/api/multipass/{}", url_component(id))
    }

    pub(crate) fn agent_card_path(id: &str) -> String {
        format!("/api/multipass/{}/agent-card", url_component(id))
    }

    pub(crate) fn x401_manifest_path(id: &str) -> String {
        format!("/api/multipass/{}/x401", url_component(id))
    }

    pub(crate) fn api_url(path: &str) -> String {
        format!("{API_BASE_URL}{path}")
    }

    pub(crate) fn multipass_url(path: &str) -> String {
        format!("{MULTIPASS_BASE_URL}{path}")
    }

    pub(crate) fn get_json(&self, path: &str) -> Result<Value, String> {
        self.get_json_url(&Self::api_url(path))
    }

    pub(crate) fn get_multipass_json(&self, path: &str) -> Result<Value, String> {
        self.get_json_url(&Self::multipass_url(path))
    }

    fn get_json_url(&self, url: &str) -> Result<Value, String> {
        let res = self
            .http
            .get(url)
            .send()
            .map_err(|err| format!("Helixa API request failed: {err}"))?;
        let status = res.status();
        let value: Value = res
            .json()
            .map_err(|err| format!("Helixa API returned invalid JSON: {err}"))?;
        if !status.is_success() {
            let msg = value
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("Helixa API error");
            return Err(format!("{msg} (HTTP {status})"));
        }
        Ok(value)
    }
}

pub(crate) fn clamp_limit(limit: Option<u32>) -> u32 {
    limit.unwrap_or(5).clamp(1, 20)
}

pub(crate) fn normalize_search(value: Value) -> Value {
    json!({
        "query": value.get("query"),
        "total": value.get("total"),
        "agents": normalize_list(value.get("agents")),
        "humans": normalize_list(value.get("humans")),
        "organizations": normalize_list(value.get("organizations")),
        "principals": normalize_list(value.get("principals")),
    })
}

fn normalize_list(value: Option<&Value>) -> Vec<Value> {
    value.and_then(Value::as_array).map(|items| {
        items.iter().map(|item| json!({
            "entity_type": item.get("entityType"),
            "id": item.get("id").or_else(|| item.get("tokenId")),
            "token_id": item.get("tokenId"),
            "name": item.get("name"),
            "framework": item.get("framework"),
            "description": item.get("description"),
            "cred_score": item.get("credScore"),
            "tier": item.get("tier"),
            "tier_label": item.get("tierLabel"),
            "verified": item.get("verified"),
            "skills": item.get("skills"),
            "service_categories": item.get("serviceCategories"),
            "suggested_actions": item.get("suggested_actions"),
            "profile_urls": {
                "profile": item.get("suggested_actions").and_then(|actions| actions.get("profile")),
                "cred": item.get("suggested_actions").and_then(|actions| actions.get("cred")),
                "card": item.get("suggested_actions").and_then(|actions| actions.get("card")),
                "public_profile": item.get("suggested_actions").and_then(|actions| actions.get("publicProfile")),
            },
        })).collect()
    }).unwrap_or_default()
}

pub(crate) fn normalize_agent_profile(value: Value) -> Value {
    json!({
        "identity": {
            "token_id": value.get("tokenId"),
            "name": value.get("name"),
            "framework": value.get("framework"),
            "mint_origin": value.get("mintOrigin"),
            "verified": value.get("verified"),
            "soulbound": value.get("soulbound"),
            "minted_at": value.get("mintedAt"),
            "generation": value.get("generation"),
        },
        "wallets": {
            "agent_address": value.get("agentAddress"),
            "owner": value.get("owner"),
            "operator": value.get("operator"),
        },
        "cred": {
            "score": value.get("credScore"),
            "points": value.get("points"),
            "ethos_score": value.get("ethosScore"),
            "talent_score": value.get("talentScore"),
        },
        "aura": {
            "personality": value.get("personality"),
            "narrative": value.get("narrative"),
        },
        "traits": value.get("traits"),
        "credentials": {
            "socials": value.get("socials"),
            "skills": value.get("skills"),
            "domains": value.get("domains"),
            "linked_token": value.get("linkedToken"),
        },
        "services": value.get("services"),
        "metadata": value.get("metadata"),
        "links": {
            "explorer": value.get("explorer"),
            "public_profile": value.get("tokenId").and_then(Value::as_u64).map(|id| format!("https://helixa.xyz/agent/{id}")),
            "api_profile": value.get("tokenId").and_then(Value::as_u64).map(|id| format!("https://api.helixa.xyz/api/v2/agent/{id}")),
        }
    })
}

pub(crate) fn normalize_cred(value: Value) -> Value {
    let score = value
        .get("credScore")
        .and_then(Value::as_i64)
        .unwrap_or_default();
    let tier = value.get("tier").and_then(Value::as_str).unwrap_or("");
    json!({
        "token_id": value.get("tokenId"),
        "name": value.get("name"),
        "score": score,
        "tier": tier,
        "tier_label": value.get("tierLabel"),
        "scale": value.get("scale"),
        "full_report_endpoint": value.get("fullReportEndpoint"),
        "recommendation": trust_recommendation(score, tier),
    })
}

pub(crate) fn normalize_multipass_profile(value: Value) -> Value {
    json!({
        "schema_version": value.get("schema_version"),
        "multipass_id": value.get("multipass_id"),
        "slug": value.get("slug"),
        "display_name": value.get("display_name"),
        "subject_type": value.get("subject_type"),
        "status": value.get("status"),
        "owner_summary": value.get("owner_summary"),
        "public_fragments": value.get("public_fragments"),
        "trust_summary": value.get("trust_summary"),
        "summary": value.get("summary"),
        "links": value.get("links"),
        "agent_card_url": link_href(&value, "agent-card"),
        "hydrated_profile_url": link_href(&value, "hydrated"),
        "x401_manifest_url": link_href(&value, "x401"),
        "x402_manifest_url": link_href(&value, "x402"),
        "boundaries": [
            "Multipass profile data is public profile metadata only.",
            "This Aomi app is read-only and cannot mutate Helixa, Multipass, wallets, or transactions."
        ],
    })
}

pub(crate) fn normalize_agent_card(value: Value) -> Value {
    json!({
        "schema_version": value.get("schema_version"),
        "multipass_id": value.get("multipass_id"),
        "name": value.get("name"),
        "subject_type": value.get("subject_type"),
        "capabilities": value.get("capabilities"),
        "message_routes": value.get("message_routes"),
        "service_endpoints": value.get("service_endpoints"),
        "services": value.get("services"),
        "trust_summary": value.get("trust_summary"),
        "contact_policy": value.get("contact_policy"),
        "standards_refs": value.get("standards_refs"),
        "accepted_assets": value.get("accepted_assets"),
        "summary": value.get("summary"),
        "links": value.get("links"),
        "x401_manifest_url": value.get("x401_manifest_url"),
        "x402_manifest_url": value.get("x402_manifest_url"),
        "routing_note": "Use this card for public discovery and routing context only; do not treat it as execution authority.",
    })
}

pub(crate) fn normalize_x401_manifest(value: Value) -> Value {
    json!({
        "schema_version": value.get("schema_version"),
        "multipass_id": value.get("multipass_id"),
        "x401_supported": value.get("x401_supported"),
        "proof_challenge_protocol": value.get("proof_challenge_protocol"),
        "current_header_names": value.get("current_header_names"),
        "trusted_issuers": value.get("trusted_issuers"),
        "proof_requirements": value.get("proof_requirements"),
        "route_policies": value.get("route_policies"),
        "boundaries": value.get("boundaries"),
        "safety_note": "Public x401 metadata describes proof requirements and route policy only. It does not expose private credentials or imply an issuer partnership.",
    })
}

fn link_href(value: &Value, rel: &str) -> Option<String> {
    value
        .get("links")
        .and_then(Value::as_array)
        .and_then(|links| {
            links.iter().find_map(|link| {
                if link.get("rel").and_then(Value::as_str) == Some(rel) {
                    link.get("href").and_then(Value::as_str).map(str::to_string)
                } else {
                    None
                }
            })
        })
}

pub(crate) fn require_public_id(id: &str) -> Result<&str, String> {
    let id = id.trim();
    if id.is_empty() {
        Err("public Multipass id is required".to_string())
    } else {
        Ok(id)
    }
}

pub(crate) fn trust_recommendation(score: i64, tier: &str) -> &'static str {
    match tier.to_ascii_uppercase().as_str() {
        "PREFERRED" => "highest-trust candidate",
        "PRIME" => "strong candidate for trusted routing",
        "QUALIFIED" => "acceptable for normal collaboration with review",
        "MARGINAL" => "use only for low-risk exploration",
        "JUNK" => "do not route paid or sensitive work",
        _ if score >= 91 => "highest-trust candidate",
        _ if score >= 76 => "strong candidate for trusted routing",
        _ if score >= 51 => "acceptable for normal collaboration with review",
        _ if score >= 26 => "use only for low-risk exploration",
        _ => "do not route paid or sensitive work",
    }
}

pub(crate) fn url_component(input: &str) -> String {
    let mut out = String::new();
    for byte in input.trim().bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_paths_are_stable() {
        assert_eq!(
            HelixaClient::search_path("bendr", 3),
            "/api/v2/search?q=bendr&limit=3"
        );
        assert_eq!(
            HelixaClient::search_path("aomi & helixa", 5),
            "/api/v2/search?q=aomi%20%26%20helixa&limit=5"
        );
        assert_eq!(HelixaClient::agent_path(1), "/api/v2/agent/1");
        assert_eq!(HelixaClient::cred_path(1), "/api/v2/agent/1/cred");
        assert_eq!(
            HelixaClient::api_url(&HelixaClient::agent_path(1)),
            "https://api.helixa.xyz/api/v2/agent/1"
        );
        assert_eq!(
            HelixaClient::multipass_profile_path("bendr-2-1"),
            "/api/multipass/bendr-2-1"
        );
        assert_eq!(
            HelixaClient::multipass_url(&HelixaClient::multipass_profile_path("bendr-2-1")),
            "https://helixa.xyz/api/multipass/bendr-2-1"
        );
        assert_eq!(
            HelixaClient::agent_card_path("mp_helixa_agent_1"),
            "/api/multipass/mp_helixa_agent_1/agent-card"
        );
        assert_eq!(
            HelixaClient::x401_manifest_path("bad/id"),
            "/api/multipass/bad%2Fid/x401"
        );
    }

    #[test]
    fn x401_manifest_normalization_preserves_public_boundaries() {
        let value = normalize_x401_manifest(json!({
            "schema_version": "0.1.0",
            "multipass_id": "mp_helixa_agent_1",
            "x401_supported": true,
            "trusted_issuers": [{ "issuer_id": "helixa", "status": "supported" }],
            "proof_requirements": [{
                "requirement_id": "human_authorization",
                "visibility": "public"
            }],
            "route_policies": [{ "route_id": "lookup", "x401_required": true }],
            "boundaries": ["Public x401 metadata does not expose private credentials."]
        }));

        assert_eq!(value["x401_supported"], true);
        assert_eq!(value["trusted_issuers"][0]["issuer_id"], "helixa");
        assert_eq!(value["proof_requirements"][0]["visibility"], "public");
        assert!(
            value["boundaries"][0]
                .as_str()
                .unwrap()
                .contains("does not expose private credentials")
        );
    }

    #[test]
    fn trust_recommendation_matches_tiers() {
        assert_eq!(
            trust_recommendation(12, "JUNK"),
            "do not route paid or sensitive work"
        );
        assert_eq!(
            trust_recommendation(45, "MARGINAL"),
            "use only for low-risk exploration"
        );
        assert_eq!(
            trust_recommendation(66, "QUALIFIED"),
            "acceptable for normal collaboration with review"
        );
        assert_eq!(
            trust_recommendation(80, "PRIME"),
            "strong candidate for trusted routing"
        );
        assert_eq!(
            trust_recommendation(95, "PREFERRED"),
            "highest-trust candidate"
        );
    }
}
