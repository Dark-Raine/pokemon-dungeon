mod pokiapi_core;

use std::io;
use std::io::Write;
use pokiapi_core::pokemon_data_structure::{PokemonData,Pokemon};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let mut requested_pokemon = String::new();
  let base_uri = "https://pokeapi.co/api/v2/pokemon/";
  println!("which pokemon would you like to know about?");
  io::stdin()
    .read_line(&mut requested_pokemon)
    .expect("failed to read");

  let resp = reqwest::get(format!("{}{}", base_uri, requested_pokemon))
    .await?
    .json::<serde_json::Value>()
    .await?;
  let pokemon: PokemonData = serde_json::from_value(resp).expect("wrong Format");
  let pokemon_parsed = Pokemon::from(pokemon);
  let mut stdout = io::stdout();
  writeln!(stdout, "{:?}", pokemon_parsed);

  Ok(())
}
