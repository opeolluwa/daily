use ironpress::{HtmlConverter, PageSize};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

use crate::error::LunarError;

pub async fn render_html_to_pdf(
    html: &str,
    filename: &str,
    output_path: &std::path::Path,
) -> Result<(), LunarError> {
    let pdf = HtmlConverter::new()
        .page_size(PageSize::A4)
        .add_font(
            "Lato",
            include_bytes!("../assets/fonts/Lato-Regular.ttf").to_vec(),
        )
        .add_font(
            "Lato Bold",
            include_bytes!("../assets/fonts/Lato-Bold.ttf").to_vec(),
        )
        .convert(html)
        .map_err(|err| LunarError::HTMLConversionError(err.to_string()))?;

    let filename = format!("{}.pdf", filename);
    let dir = PathBuf::from(output_path);
    let file_path = dir.join(filename);

    write_private_file(&file_path, &pdf)
        .await
        .map_err(|err| LunarError::HTMLConversionError(err.to_string()))?;

    Ok(())
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
