use aho_corasick::AhoCorasick;
use anyhow::Result;
use chrono::Datelike;
use ptree::TreeBuilder;
use std::io::Write;
use std::path::Path;
use tracing::instrument;

/// The MIT License.
pub(crate) const MIT_LICENSE: &str = "MIT License\n\nCopyright (c) [year] [fullname]\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\nof this software and associated documentation files (the \"Software\"), to deal\nin the Software without restriction, including without limitation the rights\nto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\ncopies of the Software, and to permit persons to whom the Software is\nfurnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all\ncopies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\nAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\nOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\nSOFTWARE.\n";

/// Helper function to build an MIT License with imputed values.
pub(crate) fn build_mit_license() -> String {
    impute_license(MIT_LICENSE)
}

/// Impute templated license strs with dynamic values.
pub(crate) fn impute_license(haystack: &str) -> String {
    let patterns = &["<year>", "[year]", "<fullname>", "[fullname]"];
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(patterns)
        .unwrap();
    let mut result = String::new();
    ac.replace_all_with(haystack, &mut result, |mat, _, dst| {
        match mat.pattern().as_usize() {
            0 | 1 => dst.push_str(&chrono::Utc::now().year().to_string()),
            2 | 3 => dst.push_str(&crate::root::get_current_username(&None)),
            _ => unreachable!(),
        }
        true
    });
    result
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
    crate::utils::create_dir_gracefully!(dir, dry);

    // Fetch the license in a tokio runtime.
    let license = match tokio::runtime::Runtime::new()?.block_on(fetch_license(license.as_ref())) {
        Ok(license) => license
            .replacen("<year>", &chrono::Utc::now().year().to_string(), 1)
            .replacen(
                "<copyright holders>",
                &crate::root::get_current_username(&None),
                1,
            ),
        Err(_) => {
            if !inquire::Confirm::new(&format!(
                "Failed to query for license \"{}\", do you want to proceed with the MIT License instead?",
                license.as_ref(),
            ))
            .prompt()?
            {
                tracing::warn!("User chose not to proceed with the MIT License");
                return Ok(());
            }
            build_mit_license()
        }
    };

    if !dry {
        tracing::debug!("Writing MIT license to {:?}", dir.join("LICENSE"));
        let mut file = std::fs::File::create(dir.join("LICENSE"))?;
        file.write_all(license.as_bytes())?;
    }

    tree.map(|t| t.add_empty_child("LICENSE".to_string()));
    Ok(())
}

/// Fetch a license using [lice].
pub(crate) async fn fetch_license(name: impl AsRef<str>) -> Result<String> {
    tracing::debug!("Fetching license from lice");
    let license = lice::get(name.as_ref()).await.map_err(|e| {
        tracing::warn!(
            "Failed to find license \"{}\" in SPDX database",
            name.as_ref()
        );
        anyhow::anyhow!(e)
    })?;
    tracing::debug!("Fetched license from lice");
    license
        .license_text
        .ok_or(anyhow::anyhow!("no license text!"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_fetch_license() {
        let license = fetch_license("mit")
            .await
            .unwrap()
            .replacen("<year>", &chrono::Utc::now().year().to_string(), 1)
            .replacen(
                "<copyright holders>",
                &crate::root::get_current_username(&None),
                1,
            );
        assert_eq!(
            license.replace("\n\n", " ").replace("\n", " "),
            build_mit_license().replace("\n\n", " ").replace("\n", " ")
        );
    }

    #[test]
    fn test_impute_license() {
        let haystack = r#"MIT License <year> <fullname>"#;
        let license = haystack
            .replacen("<year>", &chrono::Utc::now().year().to_string(), 1)
            .replacen("<fullname>", &crate::root::get_current_username(&None), 1);
        assert_eq!(license, impute_license(haystack));
    }

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
        assert_eq!(
            contents.replace("\n\n", " ").replace("\n", " "),
            build_mit_license().replace("\n\n", " ").replace("\n", " ")
        );
    }
}
