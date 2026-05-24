use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct VersionLspExtension {
    cached_binary_path: Option<String>,
}

impl VersionLspExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        let binary_settings =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)?
                .binary;
        let binary = binary_settings.and_then(|b| b.path);

        if let Some(path) = binary {
            return Ok(path);
        }

        if let Some(path) = worktree.which("version-lsp") {
            return Ok(path);
        }

        self.zed_managed_binary(language_server_id)
    }

    fn zed_managed_binary(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "skanehira/version-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let (os_name, arch_name, ext) = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => ("Darwin", "arm64", "tar.gz"),
            (zed::Os::Mac, zed::Architecture::X8664) => ("Darwin", "x86_64", "tar.gz"),
            (zed::Os::Linux, zed::Architecture::Aarch64) => ("Linux", "arm64", "tar.gz"),
            (zed::Os::Linux, zed::Architecture::X8664) => ("Linux", "x86_64", "tar.gz"),
            (zed::Os::Windows, zed::Architecture::X8664) => ("Windows", "x86_64", "zip"),
            (zed::Os::Windows, zed::Architecture::Aarch64) => {
                return Err("unsupported platform: Windows arm64".into());
            }
            (_, zed::Architecture::X86) => {
                return Err("unsupported platform: x86".into());
            }
        };

        let asset_name = format!(
            "version-lsp_{os_name}_{arch_name}.{ext}"
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("version-lsp-{}", release.version);
        let binary_name = match platform {
            zed::Os::Windows => "version-lsp.exe",
            _ => "version-lsp",
        };
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry =
                    entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for VersionLspExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary,
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)?
            .settings
            .clone();
        Ok(settings)
    }
}

zed::register_extension!(VersionLspExtension);
