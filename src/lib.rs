use zed_extension_api as zed;

struct CssVariablesExtension;

impl zed::Extension for CssVariablesExtension {
    fn new() -> Self {
        CssVariablesExtension
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if language_server_id.as_ref() != "css_variables" {
            return Err(format!("Unknown language server id: {language_server_id}"));
        }

        build_css_variables_command(worktree)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        // Return default settings matching css-variables-language-server's defaultSettings.
        // We nest them under the `cssVariables` key because the server calls
        // `connection.workspace.getConfiguration('cssVariables')`, and Zed's
        // bridge likely indexes into this object by that key.
        Ok(Some(zed::serde_json::json!({
            "cssVariables": {
                "lookupFiles": ["**/*.less", "**/*.scss", "**/*.sass", "**/*.css"],
                "blacklistFolders": [
                    "**/.cache",
                    "**/.DS_Store",
                    "**/.git",
                    "**/.hg",
                    "**/.next",
                    "**/.svn",
                    "**/bower_components",
                    "**/CVS",
                    "**/dist",
                    "**/node_modules",
                    "**/tests",
                    "**/tmp",
                ],
            }
        })))
    }
}

fn build_css_variables_command(worktree: &zed::Worktree) -> zed::Result<zed::Command> {
    // Use Zed's built-in Node & NPM helpers to install and run
    // `css-variables-language-server` in the extension's own work directory.
    let package = "css-variables-language-server";
    let version = "2.7.0";

    // Install the package if it's missing or on a different version.
    match zed::npm_package_installed_version(package)? {
        Some(installed) if installed == version => {
            // already correct version
        }
        _ => {
            zed::npm_install_package(package, version)?;
        }
    }

    let node = zed::node_binary_path()?;

    // Start with the worktree's shell environment so PATH and other vars are inherited.
    let env = worktree.shell_env();

    Ok(zed::Command {
        command: node,
        args: vec![
            "node_modules/css-variables-language-server/bin/index.js".to_string(),
            "--stdio".to_string(),
        ],
        env,
    })
}

zed::register_extension!(CssVariablesExtension);
