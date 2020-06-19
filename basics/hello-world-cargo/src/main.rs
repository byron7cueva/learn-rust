//HTTP Get request
use reqwest;
use tokio;

#[tokio::main]
async fn main () -> Result<(), reqwest::Error>{
  get().await?;
  Ok(())
}

async fn get () -> Result<(), reqwest::Error> {
  let response = reqwest::get("https://hyper.rs").await?;

  println!("Status: {}", response.status());

  let body = response.text().await?;

  println!("Body:\n\n{}", body);

  Ok(())
}