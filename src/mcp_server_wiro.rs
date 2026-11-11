use std::env;

use schemars::JsonSchema;
use serde::Deserialize;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

/// The Wiro AI MCP server on npm.
const PACKAGE_NAME: &str = "@wiro-ai/wiro-mcp";
/// The server's stdio entry point inside the extension's working directory.
const SERVER_PATH: &str = "node_modules/@wiro-ai/wiro-mcp/dist/index.js";

const MISSING_API_KEY: &str = "Add your Wiro AI API key to this server's settings as \
    `wiro_api_key`. You can find it under your project at https://wiro.ai/panel/project.";

/// Settings for the Wiro AI MCP server, read from
/// `context_servers.mcp-server-wiro.settings` in Zed's settings.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct WiroSettings {
    /// API key of your Wiro AI project. Create a project at
    /// https://wiro.ai/panel/project/new to get one.
    #[schemars(length(min = 1))]
    wiro_api_key: String,

    /// API secret of the same project. Needed only for a project with
    /// signature-based authentication; leave it empty for an API Key Only project.
    #[serde(default)]
    wiro_api_secret: Option<String>,

    /// Tool groups to load. All groups load when this is empty or left out.
    #[serde(default)]
    toolsets: Option<Vec<Toolset>>,
}

/// A group of tools the server can load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum Toolset {
    /// Search, price and run models, and manage tasks.
    Models,
    /// Build, run and resume workflows.
    Workflows,
    /// Talk to Studio and deploy and manage your own agents.
    Agents,
}

impl Toolset {
    fn as_str(self) -> &'static str {
        match self {
            Toolset::Models => "models",
            Toolset::Workflows => "workflows",
            Toolset::Agents => "agents",
        }
    }
}

/// Builds the environment for the server from the settings.
///
/// Every variable the server reads for auth and tool groups is set, even when
/// empty, so a value left over in the environment Zed was started from never
/// takes the place of what the settings say.
fn server_env(settings: WiroSettings) -> Result<Vec<(String, String)>> {
    let api_key = settings.wiro_api_key.trim().to_string();
    if api_key.is_empty() {
        return Err(MISSING_API_KEY.into());
    }

    let api_secret = settings
        .wiro_api_secret
        .map(|secret| secret.trim().to_string())
        .unwrap_or_default();

    let toolsets = settings
        .toolsets
        .unwrap_or_default()
        .iter()
        .map(|toolset| toolset.as_str())
        .collect::<Vec<_>>()
        .join(",");

    Ok(vec![
        ("WIRO_API_KEY".into(), api_key),
        ("WIRO_API_SECRET".into(), api_secret),
        ("WIRO_TOOLSETS".into(), toolsets),
    ])
}

/// Parses the `settings` object of the context server entry.
fn parse_settings(settings: Option<serde_json::Value>) -> Result<WiroSettings> {
    let Some(settings) = settings else {
        return Err(MISSING_API_KEY.into());
    };
    if settings.get("wiro_api_key").is_none() {
        return Err(MISSING_API_KEY.into());
    }
    serde_json::from_value(settings).map_err(|error| format!("Invalid Wiro AI settings: {error}"))
}

struct WiroExtension;

impl WiroExtension {
    /// Installs the server from npm, or updates it when a newer version is out.
    /// When npm cannot be reached, an installed version is used as it is.
    fn install_server(&self) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        match zed::npm_package_latest_version(PACKAGE_NAME) {
            Ok(latest_version) => {
                if installed_version.as_deref() != Some(latest_version.as_str()) {
                    if let Err(error) = zed::npm_install_package(PACKAGE_NAME, &latest_version) {
                        if installed_version.is_none() {
                            return Err(format!(
                                "Failed to install {PACKAGE_NAME}@{latest_version}: {error}"
                            ));
                        }
                    }
                }
            }
            Err(error) => {
                if installed_version.is_none() {
                    return Err(format!(
                        "Failed to look up the latest version of {PACKAGE_NAME} on npm: {error}"
                    ));
                }
            }
        }

        Ok(())
    }
}

impl zed::Extension for WiroExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;
        let env = server_env(parse_settings(settings.settings)?)?;

        self.install_server()?;

        let server_path = env::current_dir()
            .map_err(|error| format!("Failed to read the extension directory: {error}"))?
            .join(SERVER_PATH);
        if !server_path.is_file() {
            return Err(format!(
                "{PACKAGE_NAME} is not installed: {} is missing",
                server_path.display()
            ));
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path.to_string_lossy().into_owned()],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let settings_schema = serde_json::to_string(&schemars::schema_for!(WiroSettings))
            .map_err(|error| error.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!(
                "../configuration/installation_instructions.md"
            )
            .to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
            settings_schema,
        }))
    }
}

zed::register_extension!(WiroExtension);

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(json: &str) -> Result<WiroSettings> {
        parse_settings(Some(serde_json::from_str(json).unwrap()))
    }

    fn env_for(json: &str) -> Result<Vec<(String, String)>> {
        server_env(settings(json)?)
    }

    #[test]
    fn api_key_only() {
        assert_eq!(
            env_for(r#"{ "wiro_api_key": " key " }"#).unwrap(),
            vec![
                ("WIRO_API_KEY".to_string(), "key".to_string()),
                ("WIRO_API_SECRET".to_string(), String::new()),
                ("WIRO_TOOLSETS".to_string(), String::new()),
            ]
        );
    }

    #[test]
    fn key_secret_and_toolsets() {
        assert_eq!(
            env_for(
                r#"{
                    "wiro_api_key": "key",
                    "wiro_api_secret": "secret",
                    "toolsets": ["models", "workflows"]
                }"#
            )
            .unwrap(),
            vec![
                ("WIRO_API_KEY".to_string(), "key".to_string()),
                ("WIRO_API_SECRET".to_string(), "secret".to_string()),
                ("WIRO_TOOLSETS".to_string(), "models,workflows".to_string()),
            ]
        );
    }

    #[test]
    fn missing_or_blank_key_is_rejected() {
        assert_eq!(parse_settings(None).unwrap_err(), MISSING_API_KEY);
        assert_eq!(settings("{}").unwrap_err(), MISSING_API_KEY);
        assert_eq!(
            env_for(r#"{ "wiro_api_key": "  " }"#).unwrap_err(),
            MISSING_API_KEY
        );
    }

    #[test]
    fn unknown_settings_are_rejected() {
        assert!(settings(r#"{ "wiro_api_key": "key", "wiro_api_secrett": "x" }"#).is_err());
        assert!(settings(r#"{ "wiro_api_key": "key", "toolsets": ["model"] }"#).is_err());
    }

    #[test]
    fn default_settings_parse_and_ask_for_a_key() {
        let text = include_str!("../configuration/default_settings.jsonc");
        let value: serde_json::Value = serde_json_lenient::from_str(text).unwrap();
        let settings = parse_settings(Some(value)).unwrap();
        assert_eq!(server_env(settings).unwrap_err(), MISSING_API_KEY);

        // Filling in the key and uncommenting the optional line still parses.
        let filled = text
            .replacen(r#""wiro_api_key": """#, r#""wiro_api_key": "key""#, 1)
            .replacen(r#"// "toolsets""#, r#""toolsets""#, 1);
        let value: serde_json::Value = serde_json_lenient::from_str(&filled).unwrap();
        assert_eq!(
            server_env(parse_settings(Some(value)).unwrap()).unwrap()[2],
            (
                "WIRO_TOOLSETS".to_string(),
                "models,workflows,agents".to_string()
            )
        );
    }

    #[test]
    fn schema_requires_a_non_empty_key() {
        let schema = serde_json::to_value(schemars::schema_for!(WiroSettings)).unwrap();
        assert_eq!(schema["required"], serde_json::json!(["wiro_api_key"]));
        assert_eq!(schema["properties"]["wiro_api_key"]["minLength"], 1);
        assert_eq!(schema["additionalProperties"], false);
    }
}
