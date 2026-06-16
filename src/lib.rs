use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, GithubReleaseOptions,
    Project, Result,
};

mod binary;
mod platform;

struct MnemodocExtension;

fn resolve_binary(work_dir: &str) -> Result<String> {
    if let Some(path) = binary::system_path() {
        return Ok(path);
    }

    if let Some(path) = binary::local_path(work_dir) {
        return Ok(path);
    }

    let (os, arch) = zed::current_platform();
    let os_str = match os {
        zed::Os::Mac => "macos",
        zed::Os::Linux => "linux",
        zed::Os::Windows => return Err("Unsupported platform: Windows".into()),
    };
    let arch_str = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x86_64",
        zed::Architecture::X86 => return Err("Unsupported architecture: x86".into()),
    };

    let asset_name = platform::asset_name(os_str, arch_str)?;

    let release = zed::latest_github_release(
        platform::SERVER_REPO,
        GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;
    let assets: Vec<(&str, &str)> = release
        .assets
        .iter()
        .map(|a| (a.name.as_str(), a.download_url.as_str()))
        .collect();
    let url = platform::select_asset_url(&assets, &asset_name)?;

    zed::download_file(url, binary::BINARY_NAME, zed::DownloadedFileType::Uncompressed)
        .map_err(|e| format!("Failed to download mnemodoc-server: {e}"))?;
    zed::make_file_executable(binary::BINARY_NAME)
        .map_err(|e| format!("Failed to make binary executable: {e}"))?;

    Ok(format!("{work_dir}/{}", binary::BINARY_NAME))
}

impl zed::Extension for MnemodocExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        // Zed pins the WASI CWD to the extension's working directory ($PWD) and
        // forbids chdir, so current_dir() is the extension work dir — the base
        // download_file writes to and where a downloaded binary lives.
        let work_dir = std::env::current_dir()
            .map_err(|e| e.to_string())?
            .to_str()
            .ok_or("work directory path contains invalid UTF-8")?
            .to_string();

        let binary = resolve_binary(&work_dir)?;

        // The WASM sandbox cannot reach the project root, so the extension never
        // touches `.mnemodoc.yml`. Zed spawns this command with its CWD set to
        // the project root, and the server resolves its default config path
        // (`.mnemodoc.yml`) — or its built-in defaults when absent — against
        // that CWD on its own. Passing `--config` here would re-anchor the
        // server to the extension work dir and break path resolution.
        Ok(Command {
            command: binary,
            args: vec!["serve".into(), "--stdio".into()],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        // All configuration lives in the project's `.mnemodoc.yml`, read by the
        // server itself; the extension exposes no Zed settings of its own.
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings: "{}".to_string(),
            settings_schema: "{}".to_string(),
        }))
    }
}

zed::register_extension!(MnemodocExtension);
