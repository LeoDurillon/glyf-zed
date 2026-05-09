use std::path::Path;

use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json};

const LSP_VERSION: &str = "0.1.0";
const RELEASE_BASE: &str = "https://github.com/LeoDurillon/glyf-lsp/releases/download";

struct GlyfExtension {
    cached_binary_path: Option<String>,
}

impl GlyfExtension {
    fn check_binary_exist(&self) -> Option<String> {
        let (platform, _) = zed::current_platform();

        let binary_name = match platform {
            zed::Os::Windows => "glyf-lsp.exe",
            _ => "glyf-lsp",
        };

        let binary_path = format!("glyf-lsp/{LSP_VERSION}/{binary_name}");
        Path::new(&binary_path).exists().then_some(binary_path)
    }

    fn get_archive_info(&self) -> Result<(String, zed::DownloadedFileType, String)> {
        let (platform, arch) = zed::current_platform();

        let target = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "aarch64-apple-darwin",
            (zed::Os::Mac, zed::Architecture::X8664) => "x86_64-apple-darwin",
            (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-unknown-linux-gnu",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => return Err(format!("unsupported platform: {platform:?}/{arch:?}")),
        };

        return match platform {
            zed::Os::Windows => Ok((
                format!("glyf-lsp-v{LSP_VERSION}-{target}.zip"),
                zed::DownloadedFileType::Zip,
                "glyf-lsp.exe".to_string(),
            )),
            _ => Ok((
                format!("glyf-lsp-v{LSP_VERSION}-{target}.tar.gz"),
                zed::DownloadedFileType::GzipTar,
                "glyf-lsp".to_string(),
            )),
        };
    }

    fn download_archive(&self, language_server_id: &LanguageServerId) -> Result<String> {
        let (archive, file_type, binary_name) = self.get_archive_info()?;

        let url = format!("{RELEASE_BASE}/v{LSP_VERSION}/{archive}");
        let install_path = format!("glyf-lsp/{LSP_VERSION}");
        let binary_path = format!("glyf-lsp/{LSP_VERSION}/{binary_name}");

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(&url, &install_path, file_type)
            .map_err(|e| format!("failed to download glyf-lsp: {e}"))?;

        zed::make_file_executable(&binary_path)
            .map_err(|e| format!("failed to make glyf-lsp executable: {e}"))?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        return Ok(binary_path);
    }
}

impl zed::Extension for GlyfExtension {
    fn new() -> Self {
        GlyfExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Zed passes the user's `lsp.glyf.initialization_options`
        // from settings.json automatically — no action needed here.
        Ok(None)
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = &self.cached_binary_path {
            return Ok(zed::Command {
                command: path.clone(),
                args: vec![],
                env: Default::default(),
            });
        }

        let binary_path = match self.check_binary_exist() {
            Some(path) => path,
            None => self.download_archive(language_server_id)?,
        };
        self.cached_binary_path = Some(binary_path.clone());

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GlyfExtension);
