use chrono::Datelike;
use eyre::Result;
use ptree::TreeBuilder;
use std::io::Write;
use std::path::Path;
use tracing::instrument;

/// The MIT License.
pub(crate) const MIT_LICENSE: &str = "MIT License\n\nCopyright (c) [year] [fullname]\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\nof this software and associated documentation files (the \"Software\"), to deal\nin the Software without restriction, including without limitation the rights\nto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\ncopies of the Software, and to permit persons to whom the Software is\nfurnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all\ncopies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\nAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\nOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\nSOFTWARE.\n";

/// Builds the contents of the license file, performing template substitutions.
#[instrument(name = "license", skip(license))]
pub(crate) fn build(license: impl AsRef<str>) -> String {
    tracing::debug!("Building license file contents");
    #[allow(clippy::match_single_binding)]
    match license.as_ref() {
        // Default to the MIT License
        // todo: Add support for other licenses.
        _ => {
            // Replace the `[year]` and `[fullname]` placeholders with the current year and the
            // current user's full name.
            MIT_LICENSE
                .replace("[year]", &chrono::Utc::now().year().to_string())
                .replace("[fullname]", &crate::root::get_current_username(&None))
        }
    }
}

/// Creates a new license file in the given directory.
#[instrument(name = "license", skip(dir, license, dry, tree))]
pub(crate) fn create(
    dir: &Path,
    license: impl AsRef<str>,
    dry: bool,
    tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating license file");

    // Create the directory if it doesn't exist.
    if !dry {
        tracing::debug!("Creating directory {:?}", dir);
        std::fs::create_dir_all(dir)?;
    }

    // Prompt the user that the license is not supported
    // and to fall back on the MIT license.
    if !dry && license.as_ref() != "mit" {
        tracing::warn!("License {} is not supported", license.as_ref());
        if !inquire::Confirm::new(&format!(
            "License {} is not supported, do you want to proceed with the MIT License instead?",
            license.as_ref(),
        ))
        .prompt()?
        {
            return Ok(());
        }
    }

    if !dry {
        tracing::debug!("Writing MIT license to {:?}", dir.join("LICENSE"));
        let mut file = std::fs::File::create(dir.join("LICENSE"))?;
        file.write_all(build("mit").as_bytes())?;
    }

    tree.map(|t| t.add_empty_child("LICENSE".to_string()));
    Ok(())
}

/// Fetches the license file from the github api with an mit license fallback.
#[allow(dead_code)]
pub(crate) async fn fetch_with_fallback(name: impl AsRef<str>) -> Result<String> {
    match fetch_license(name.as_ref()).await {
        Ok(license) => Ok(license),
        Err(e) => {
            tracing::error!("Failed to fetch license from github api: {}", e);
            tracing::error!("Falling back to MIT license");
            if !inquire::Confirm::new(&format!(
                "Failed to fetch {} license, do you want to proceed with the MIT License instead?",
                name.as_ref(),
            ))
            .prompt()?
            {
                return Ok("".to_string());
            }
            tracing::debug!("Fetching MIT license from github api");
            fetch_license("mit").await
        }
    }
}

/// Fetches the license file from the github api.
#[instrument(name = "license", skip(name))]
pub async fn fetch_license(name: impl AsRef<str>) -> Result<String> {
    tracing::debug!("Fetching license from github api");
    let fetch_url = format!("https://api.github.com/licenses/{}", name.as_ref());
    let license = reqwest::get(fetch_url)
        .await?
        .json::<serde_json::Value>()
        .await?
        .get("body")
        .ok_or(eyre::eyre!("Failed to get body from github api response"))?
        .as_str()
        .ok_or(eyre::eyre!("Failed to convert github api body to string"))?
        .to_string();
    tracing::debug!("Fetched license from github api");
    Ok(license)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_create_license() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create(&package_dir, "mit", false, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join("LICENSE").exists());

        let mut file = File::open(package_dir.join("LICENSE")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, build("mit"));
    }
}
