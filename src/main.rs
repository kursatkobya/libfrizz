
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    libfrizz::get_header();
    Ok(())
}
