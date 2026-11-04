// Copyright (C) 2026 AnalyseDeCircuit
// SPDX-License-Identifier: GPL-3.0-only

use super::*;
use semver::{Version, VersionReq};

pub(crate) fn host_version_matches(requirement: Option<&str>, host: &Version) -> bool {
    requirement.is_none_or(|requirement| {
        VersionReq::parse(requirement).is_ok_and(|range| {
            if range.matches(host) {
                return true;
            }
            // semver 的 VersionReq 默认不匹配带预发布段的版本。本分支的版本号是
            // 2.3.0-bk.x 这类预发布形式，`>=2.1` 会整片失效，把插件全部判成
            // 「宿主版本过低」。去掉预发布段按基础版本再比一次，语义上才是对的：
            // 插件的兼容性要求针对的是 2.3.0 这个版本，不是我们的分支后缀。
            //
            // 但要求里自己写了预发布时（例如 `>2.3.0-bk.0.0.1`），作者明确是在
            // 针对预发布版本做判断，必须保持严格匹配，不能走这个兜底。
            if range.comparators.iter().any(|c| !c.pre.is_empty()) {
                return false;
            }
            let mut release = host.clone();
            release.pre = semver::Prerelease::EMPTY;
            range.matches(&release)
        })
    })
}

/// Rechecked from the manifest on each launch, including after app downgrades.
pub fn validate_native_plugin_host(manifest: &NativePluginManifest) -> Result<(), String> {
    let host = Version::parse(env!("CARGO_PKG_VERSION")).expect("valid package version");
    let requirement = manifest
        .engines
        .as_ref()
        .and_then(|engines| engines.oxideterm.as_deref());
    if host_version_matches(requirement, &host) {
        Ok(())
    } else {
        Err(format!(
            "plugin_host_incompatible: requires OxideTerm {}, current {host}",
            requirement.unwrap_or_default()
        ))
    }
}

pub(crate) fn catalog_cache_path(settings_path: &Path) -> PathBuf {
    settings_path
        .parent()
        .unwrap_or(settings_path)
        .join("plugin-catalog-cache.json")
}

pub(crate) fn load_catalog_cache(
    settings_path: &Path,
    installed_ids: &[&str],
) -> Result<Option<NativePluginRegistryIndex>, String> {
    let path = catalog_cache_path(settings_path);
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("Cannot read plugin catalog cache: {error}")),
    };
    use std::io::Read as _;
    let mut bytes = Vec::new();
    file.take(registry::NATIVE_PLUGIN_REGISTRY_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > registry::NATIVE_PLUGIN_REGISTRY_MAX_BYTES {
        return Err("Plugin catalog cache exceeds size limit".into());
    }
    let mut catalog: NativePluginRegistryIndex = serde_json::from_slice(&bytes)
        .map_err(|error| format!("Invalid plugin catalog cache: {error}"))?;
    registry::validate_native_plugin_registry(&catalog)?;
    for entry in &mut catalog.plugins {
        if installed_ids.contains(&entry.id.as_str()) && entry.history_pending() {
            if let Some(history) = catalog::load_cached_history(settings_path, entry)? {
                *entry = history;
            }
        }
    }
    Ok(Some(catalog))
}

/// Catalog corrections change only the in-memory requirement, never package contents.
pub(crate) fn apply_catalog_compatibility(
    manifest: &mut NativePluginManifest,
    catalog: &NativePluginRegistryIndex,
) {
    if let Some(release) = catalog
        .plugins
        .iter()
        .find(|entry| entry.id == manifest.id)
        .and_then(|entry| {
            entry
                .releases
                .iter()
                .find(|release| release.version == manifest.version)
        })
    {
        manifest.engines = Some(release.effective_engines().clone());
    }
}
