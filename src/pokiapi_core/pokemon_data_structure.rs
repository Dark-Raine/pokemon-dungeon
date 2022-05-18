use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct PokemonData {
  pub id: u32,
  pub name: String,
  pub weight: u32,
  pub height: u32,
  pub stats: Vec<PokemonStat>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonStat {
  #[serde(rename = "base_stat")]
  pub value: u32,
  pub stat: PokemonStatName,
}

#[derive(Deserialize, Debug)]
pub struct PokemonStatName {
  pub name: String,
}
