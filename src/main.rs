
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let resp = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto")
    .await?
    .json::<serde_json::Value>()
    .await?;
  println!("{:#?}",resp);
  Ok(())
}
