use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pokemon {
  id: i32,
  name: String,
  weight: i32,
  height: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let resp = reqwest::get("https://pokeapi.co/api/v2/pokemon/mew")
    .await?
    .json::<serde_json::Value>()
    .await?;
    let mew: Pokemon = serde_json::from_value(resp).unwrap();

  println!("{:?}",mew);
  Ok(())
}
