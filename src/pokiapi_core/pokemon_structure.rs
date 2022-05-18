use serde::{Deserialize, Serialize};
use super::pokemon_data_structure::{PokemonData};

#[derive(Deserialize, Debug, Serialize)]
pub struct Pokemon {
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
    let mut stats = i.stats.iter();
    // the use of unwrap here is correct, since those values in stats should always exist
    // as per pokeapi.co's documentation
    let hp = stats.find(|&stat| stat.stat.name == "hp").unwrap();
    let attack = stats.find(|&stat| stat.stat.name == "attack").unwrap();
    let defense = stats.find(|&stat| stat.stat.name == "defense").unwrap();

    Pokemon {
      id: i.id,
      name: i.name,
      weight: i.weight,
      height: i.height,
      hp: hp.value,
      attack: attack.value,
      defense: defense.value,
    }
  }
}