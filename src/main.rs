use std::io;
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
struct PokemonData {
  id: u32,
  name: String,
  weight: u32,
  height: u32,
  stats: Vec<PokemonStat>,
}

#[derive(Deserialize,Debug)]
struct PokemonStat {
  #[serde(rename = "base_stat")]
  value: u32,
  stat: PokemonStatName,
}

#[derive(Deserialize, Debug)]
struct PokemonStatName {
  name: String
}
#[derive(Deserialize, Debug)]
struct Pokemon {
  id: u32,
  name: String,
  weight: u32,
  height: u32,
  hp: u32,
  attack: u32,
  defense: u32,
}

impl From<PokemonData> for Pokemon {
  fn from(i: PokemonData) -> Pokemon {
    Pokemon {
      id: i.id,
      name: i.name,
      weight: i.weight,
      height: i.height,
      hp: i.stats[0].value,
      attack: i.stats[1].value,
      defense: i.stats[2].value,
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let mut requested_pokemon = String::new();
  let base_uri = "https://pokeapi.co/api/v2/pokemon/";
  println!("which pokemon would you like to know about?");
  io::stdin().read_line(&mut requested_pokemon).expect("failed to read");

  let resp = reqwest::get(format!("{}{}",base_uri,requested_pokemon))
    .await?
    .json::<serde_json::Value>()
    .await?;
    let pokemon: PokemonData = serde_json::from_value(resp).expect("wrong Format");
    let pokemon_parsed = Pokemon::from(pokemon);

  println!("{:?}",pokemon_parsed);
  Ok(())
}
