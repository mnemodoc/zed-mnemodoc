/// Name of the `mnemodoc-server` binary.
pub const BINARY_NAME: &str = "mnemodoc-server";

/// Returns the path to a system-installed `mnemodoc-server` binary, if found.
///
/// Checks common install locations used by Homebrew (macOS/Linux) and
/// system package managers. Uses filesystem existence checks compatible
/// with Zed's WASI runtime.
pub fn system_path() -> Option<String> {
    system_path_from(&[
        "/usr/local/bin/mnemodoc-server",
        "/opt/homebrew/bin/mnemodoc-server",
        "/home/linuxbrew/.linuxbrew/bin/mnemodoc-server",
        "/usr/bin/mnemodoc-server",
    ])
}

/// Returns the first path in `candidates` that exists on disk.
pub(crate) fn system_path_from(candidates: &[&str]) -> Option<String> {
    candidates
        .iter()
        .copied()
        .find(|p| std::path::Path::new(p).exists())
        .map(str::to_string)
}

/// Returns the path to a previously downloaded binary in `work_dir`, if it exists.
pub fn local_path(work_dir: &str) -> Option<String> {
    let path = format!("{work_dir}/{BINARY_NAME}");
    if std::path::Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_path_returns_none_when_absent() {
        let tmp = tempfile::tempdir().unwrap();
        let result = local_path(tmp.path().to_str().unwrap());
        assert!(result.is_none());
    }

    #[test]
    fn local_path_returns_path_when_present() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = tmp.path().join(BINARY_NAME);
        std::fs::write(&bin, b"fake binary").unwrap();
        let result = local_path(tmp.path().to_str().unwrap());
        assert!(result.is_some());
        assert!(result.unwrap().ends_with(BINARY_NAME));
    }

    #[test]
    fn system_path_from_returns_first_existing() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = tmp.path().join(BINARY_NAME);
        std::fs::write(&bin, b"").unwrap();
        let bin_str = bin.to_str().unwrap();
        let result = system_path_from(&["/nonexistent/path", bin_str]);
        assert_eq!(result.as_deref(), Some(bin_str));
    }

    #[test]
    fn system_path_from_returns_none_when_all_absent() {
        let result = system_path_from(&["/nonexistent1", "/nonexistent2"]);
        assert!(result.is_none());
    }
}
