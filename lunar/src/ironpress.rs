use ironpress::{HtmlConverter, Margin, PageSize};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

use crate::error::LunarError;
use crate::utils::detect_duplicate;

const FONT_FAMILY: &str = "Lato";
const FONT_REGULAR: &[u8] = include_bytes!("../assets/fonts/Lato-Regular.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../assets/fonts/Lato-Bold.ttf");

pub async fn render_html_to_pdf(
    html: &str,
    filename: &str,
    output_path: &Path,
    overwrite: bool,
) -> Result<PathBuf, LunarError> {
    let document = format!(r#"<div style="font-family: '{FONT_FAMILY}', sans-serif">{html}</div>"#);

    let pdf = HtmlConverter::new()
        .page_size(PageSize::A4)
        .margin(Margin::uniform(5.0))
        .add_font(FONT_FAMILY, FONT_REGULAR.to_vec())
        .add_font("Lato__bold", FONT_BOLD.to_vec())
        .convert(&document)
        .map_err(|err| LunarError::HTMLConversionError(err.to_string()))?;

    let file_path = output_path.join(format!("{}.pdf", filename));

    if let Some(existing) = detect_duplicate(&file_path)
        .await
        .map_err(|err| LunarError::HTMLConversionError(err.to_string()))?
    {
        if !overwrite {
            return Err(LunarError::FileExists(existing.display().to_string()));
        }

        // `remove_file` unlinks a symlink itself instead of following it, so
        // the `create_new` below can never be tricked into writing elsewhere.
        tokio::fs::remove_file(&existing)
            .await
            .map_err(|err| LunarError::HTMLConversionError(err.to_string()))?;
    }

    write_private_file(&file_path, &pdf)
        .await
        .map_err(|err| LunarError::HTMLConversionError(err.to_string()))?;

    Ok(file_path)
}

/// Writes a new file with owner-only permissions.
///
/// `create_new` refuses to follow or reuse an existing path, so a name that
/// collides with an attacker-planted file or symlink fails instead of
/// overwriting it.
async fn write_private_file(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let mut options = tokio::fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    {
        options.mode(0o600);
    }

    let mut file = options.open(path).await?;
    file.write_all(contents).await?;
    file.flush().await?;

    Ok(())
}
