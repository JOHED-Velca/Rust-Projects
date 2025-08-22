use anyhow::Result;

mod rag;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    println!("Rusty Assistant (RAG w/ Ollama + PDFs");
    // TODO: CLI/REPL parse;
    Ok(())
}
