/// GitHub repository hosting `mnemodoc-server` releases.
pub const SERVER_REPO: &str = "mnemodoc/mcp-server";

/// Selects the download URL of the asset named `name` from `(name, url)` pairs.
///
/// # Errors
///
/// Returns an error naming the asset if no matching entry is found.
pub fn select_asset_url<'a>(assets: &[(&'a str, &'a str)], name: &str) -> Result<&'a str, String> {
    assets
        .iter()
        .find(|(asset_name, _)| *asset_name == name)
        .map(|(_, url)| *url)
        .ok_or_else(|| format!("Release asset not found: {name}"))
}

/// Returns the GitHub Release asset name for the given OS and architecture.
///
/// The asset name matches the naming convention used in `mnemodoc-server`
/// GitHub Releases (e.g. `mnemodoc-server-darwin-arm64`).
///
/// # Errors
///
/// Returns an error string if the platform combination is not supported.
pub fn asset_name(os: &str, arch: &str) -> Result<String, String> {
    match (os, arch) {
        ("macos", "aarch64") => Ok("mnemodoc-server-darwin-arm64".to_string()),
        ("macos", "x86_64") => Ok("mnemodoc-server-darwin-amd64".to_string()),
        ("linux", "aarch64") => Ok("mnemodoc-server-linux-arm64".to_string()),
        ("linux", "x86_64") => Ok("mnemodoc-server-linux-amd64".to_string()),
        _ => Err(format!("Unsupported platform: {os}-{arch}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macos_arm64() {
        assert_eq!(asset_name("macos", "aarch64").unwrap(), "mnemodoc-server-darwin-arm64");
    }

    #[test]
    fn macos_x86_64() {
        assert_eq!(asset_name("macos", "x86_64").unwrap(), "mnemodoc-server-darwin-amd64");
    }

    #[test]
    fn linux_arm64() {
        assert_eq!(asset_name("linux", "aarch64").unwrap(), "mnemodoc-server-linux-arm64");
    }

    #[test]
    fn linux_x86_64() {
        assert_eq!(asset_name("linux", "x86_64").unwrap(), "mnemodoc-server-linux-amd64");
    }

    #[test]
    fn unsupported_returns_error() {
        assert!(asset_name("windows", "x86_64").is_err());
        assert!(asset_name("macos", "x86").is_err());
    }

    #[test]
    fn select_asset_url_returns_matching_url() {
        let assets = [
            ("mnemodoc-server-linux-x86_64", "https://example.com/linux"),
            ("mnemodoc-server-darwin-arm64", "https://example.com/darwin"),
        ];
        let url = select_asset_url(&assets, "mnemodoc-server-darwin-arm64").unwrap();
        assert_eq!(url, "https://example.com/darwin");
    }

    #[test]
    fn select_asset_url_errors_when_asset_missing() {
        let assets = [("other-asset", "https://example.com/other")];
        let result = select_asset_url(&assets, "mnemodoc-server-darwin-arm64");
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("mnemodoc-server-darwin-arm64"),
            "error should name the missing asset"
        );
    }
}
