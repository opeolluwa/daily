use ironpress::HtmlConverter;

use crate::error::LunarError;

pub fn parse_markdown_to_pdf(markdown: &str, file_name: &str) -> Result<(), LunarError> {
    let safe_name: String = file_name
        .chars()
        .filter(|c| !matches!(c, '/' | '\\' | '\0'))
        .collect::<String>();
    let safe_name = safe_name.trim();
    let safe_name = if safe_name.is_empty() {
        "untitled"
    } else {
        safe_name
    };

    let output_path = dirs::download_dir()
        .ok_or(LunarError::DownloadDirNotFound)?
        .join(safe_name)
        .with_extension("pdf");

    let pdf_bytes = HtmlConverter::new()
        .convert_markdown(markdown)
        .map_err(|e| LunarError::Markdown2Pdf(e.to_string()))?;

    std::fs::write(&output_path, pdf_bytes).map_err(|e| LunarError::Markdown2Pdf(e.to_string()))?;

    Ok(())
}
