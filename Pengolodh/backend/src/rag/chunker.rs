use anyhow::Result;
pub fn extract_text_from_pdf(_path: &str) -> Result<String> {
    // TODO: implement via lopdf or pdf-extract crate
    Ok(String::new())
}

pub fn chunk_text(_text: &str, _chunk_size: usize, _overlap: usize) -> Vec<String> {
    // TODO: implement basic word/token-based splitter
    vec![]
}