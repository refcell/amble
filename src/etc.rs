use anyhow::Result;
use image::ImageFormat;
use ptree::TreeBuilder;
use std::path::Path;
use tracing::instrument;

/// The template banner png url.
pub(crate) const BANNER_URL: &str =
    "https://raw.githubusercontent.com/refcell/amble/main/etc/template/banner.png";

/// The template logo png url.
pub(crate) const LOGO_URL: &str =
    "https://raw.githubusercontent.com/refcell/amble/main/etc/template/logo.png";

/// The template favicon ico url.
pub(crate) const FAVICON_URL: &str =
    "https://raw.githubusercontent.com/refcell/amble/main/etc/template/favicon.ico";

/// Creates a new etc directory in the given directory.
#[instrument(name = "etc", skip(dir, dry, assets, tree))]
pub(crate) fn create(
    dir: &Path,
    dry: bool,
    assets: bool,
    mut tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating etc directory");
    crate::utils::create_dir_gracefully!(dir.join("etc"), dry);
    tree.as_deref_mut()
        .map(|t| t.begin_child("etc".to_string()));
    if !dry && assets {
        tracing::debug!("Copying assets to etc directory");
        let banner_img_bytes = reqwest::blocking::get(BANNER_URL)?.bytes()?;
        let image = image::load_from_memory_with_format(&banner_img_bytes, ImageFormat::Png)?;
        image.save_with_format(dir.join("etc").join("banner.png"), ImageFormat::Png)?;
        let logo_img_bytes = reqwest::blocking::get(LOGO_URL)?.bytes()?;
        let image = image::load_from_memory_with_format(&logo_img_bytes, ImageFormat::Png)?;
        image.save_with_format(dir.join("etc").join("logo.png"), ImageFormat::Png)?;
        let favicon_img_bytes = reqwest::blocking::get(FAVICON_URL)?.bytes()?;
        let image = image::load_from_memory_with_format(&favicon_img_bytes, ImageFormat::Ico)?;
        image.save_with_format(dir.join("etc").join("favicon.ico"), ImageFormat::Ico)?;
        tree.as_deref_mut()
            .map(|t| t.add_empty_child("banner.png".to_string()));
        tree.as_deref_mut()
            .map(|t| t.add_empty_child("logo.png".to_string()));
        tree.as_deref_mut()
            .map(|t| t.add_empty_child("favicon.ico".to_string()));
    }
    tree.map(|t| t.end_child());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_etc() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create(&package_dir, false, false, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join("etc").exists());

        // Check that the etc directory is an empty directory
        assert!(package_dir.join("etc").read_dir().unwrap().next().is_none());
    }

    #[test]
    fn test_create_etc_with_assets() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create(&package_dir, false, true, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join("etc").exists());
        assert!(package_dir.join("etc").join("banner.png").exists());
        assert!(package_dir.join("etc").join("logo.png").exists());
        assert!(package_dir.join("etc").join("favicon.ico").exists());
    }
}
