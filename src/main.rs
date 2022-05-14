use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let resp = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto");
  Ok(())
}
