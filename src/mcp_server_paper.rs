use std::env;
use std::fs;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, DownloadedFileType, Project, Result};

const PAPER_MCP_URL: &str = "http://127.0.0.1:29979/mcp";
const PACKAGE_NAME: &str = "mcp-remote";
const REGISTRY_URL: &str = "https://registry.npmjs.org/mcp-remote/latest";

struct PaperModelContextExtension {
    cached_server_path: Option<String>,
}

impl PaperModelContextExtension {
    fn get_latest_version(&self) -> Result<String> {
        // Fetch version directly from npm registry via HTTP to avoid npm/npx shell calls
        let response = zed::http_client::fetch(&zed::http_client::HttpRequest {
            url: REGISTRY_URL.to_string(),
            headers: vec![],
            body: None,
            method: zed::http_client::HttpMethod::Get,
            redirect_policy: zed::http_client::RedirectPolicy::FollowAll,
        })?;
        
        let body_str = String::from_utf8(response.body)
            .map_err(|e| format!("invalid utf-8 response from npm registry: {}", e))?;
        
        let json: serde_json::Value = serde_json::from_str(&body_str)
            .map_err(|e| format!("failed to parse npm registry response: {}", e))?;
        
        json.get("version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "version not found in npm registry response".into())
    }

    fn server_script_path(&mut self, _context_server_id: &ContextServerId) -> Result<String> {
        if let Some(path) = &self.cached_server_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // Get latest version from npm registry via HTTP (no npm/npx calls)
        let latest_version = self.get_latest_version()?;
        let version_dir = format!("{}-{}", PACKAGE_NAME, latest_version);
        let script_path = format!("{}/package/dist/proxy.js", version_dir);

        // Check if already downloaded
        if !fs::metadata(&script_path).is_ok_and(|stat| stat.is_file()) {
            // Download tarball from npm registry
            let tarball_url = format!(
                "https://registry.npmjs.org/{}/-/{}-{}.tgz",
                PACKAGE_NAME, PACKAGE_NAME, latest_version
            );

            fs::create_dir_all(&version_dir)
                .map_err(|e| format!("failed to create directory '{}': {}", version_dir, e))?;

            zed::download_file(&tarball_url, &version_dir, DownloadedFileType::GzipTar)
                .map_err(|e| format!("failed to download {}: {}", PACKAGE_NAME, e))?;

            // Clean up old versions
            let entries = fs::read_dir(".")
                .map_err(|e| format!("failed to list working directory: {}", e))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry: {}", e))?;
                let name = entry.file_name();
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with(&format!("{}-", PACKAGE_NAME)) && name_str != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        self.cached_server_path = Some(script_path.clone());
        Ok(script_path)
    }
}

impl zed::Extension for PaperModelContextExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let server_script = self.server_script_path(context_server_id)?;
        
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_script)
                    .to_string_lossy()
                    .to_string(),
                PAPER_MCP_URL.into(),
            ],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<zed::ContextServerConfiguration>> {
        Ok(Some(zed::ContextServerConfiguration {
            installation_instructions: include_str!("../configuration/installation_instructions.md").to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
            settings_schema: "{}".to_string(),
        }))
    }
}

zed::register_extension!(PaperModelContextExtension);
