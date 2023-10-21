use chrono::Datelike;
use eyre::Result;
use ptree::TreeBuilder;
use std::io::Write;
use std::path::Path;
use tracing::instrument;

/// Creates a new license file in the given directory.
#[instrument(name = "license", skip(dir, license, dry, tree))]
pub(crate) fn create(
    dir: &Path,
    license: impl AsRef<str>,
    dry: bool,
    tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating license file");

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

    let mit_license = "MIT License\n\nCopyright (c) [year] [fullname]\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\nof this software and associated documentation files (the \"Software\"), to deal\nin the Software without restriction, including without limitation the rights\nto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\ncopies of the Software, and to permit persons to whom the Software is\nfurnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all\ncopies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\nAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\nOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\nSOFTWARE.\n";
    if !dry {
        // Replace the `[year]` and `[fullname]` placeholders with the current year and the
        // current user's full name.
        let mit_license = mit_license
            .replace("[year]", &chrono::Utc::now().year().to_string())
            .replace("[fullname]", &whoami::realname());
        tracing::debug!("Writing MIT license to {:?}", dir.join("LICENSE"));
        let mut file = std::fs::File::create(dir.join("LICENSE"))?;
        file.write_all(mit_license.as_bytes())?;
    }

    // We can't do this dynamic fetching since the github rest api
    // requires a token.

    // Fetch the MIT license from the github api.
    // Block the runtime on the license fetching
    // operation.
    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .thread_stack_size(8 * 1024 * 1024)
    //     .build()?;
    // let license_name = name.as_ref().to_string();
    // let license = runtime.block_on(fetch_with_fallback(license_name))?;

    // if !dry {
    //     // write the license to the LICENSE file in the project directory.
    //     tracing::debug!("Writing license to {:?}", dir.join("LICENSE"));
    //     let mut file = std::fs::File::create(dir.join("LICENSE"))?;
    //     file.write_all(license.as_bytes())?;
    // }

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_fetch_license() {
//         let license = fetch_license("mit").unwrap();
//         assert!(license.contains("MIT License"));
//     }
// }
