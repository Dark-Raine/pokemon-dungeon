use std::io;
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
  let mut requestedPokemon = String::new();
  let baseUri = "https://pokeapi.co/api/v2/pokemon/";
  println!("which pokemon would you like to know about?");
  io::stdin().read_line(&mut requestedPokemon).expect("failed to read");

  let resp = reqwest::get(format!("{}{}",baseUri,requestedPokemon))
    .await?
    .json::<serde_json::Value>()
    .await?;
    let mew: Pokemon = serde_json::from_value(resp).unwrap();

  println!("{:?}",mew);
  Ok(())
}
