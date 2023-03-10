use std::fs::File;

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KanaSymbol {
    display: String,
    translations: Vec<String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SymbolFamily {
    family: String,
    symbols: Vec<KanaSymbol>
}

impl SymbolFamily {
    pub fn random_symbol<'a>(self: &'a Self) -> &'a KanaSymbol {
        self.symbols.choose(&mut rand::thread_rng()).expect("cannot get symbol from empty family")
    }
}


impl KanaSymbol {
    pub fn new(display: &String, translations: &Vec<String>) -> Self {
        KanaSymbol {
            display: display.clone(),
            translations: translations.clone()
        }
    }

    pub fn new_single(display: &String, translation: &String) -> Self {
        KanaSymbol {
            display: display.clone(),
            translations: vec![translation.clone()]
        }
    }

    pub fn get_display(self: &Self) -> &String {
        &self.display
    }

    pub fn get_translations(self: &Self) -> &Vec<String> {
        &self.translations
    }
}

pub fn parse_symbols_from_file(filename: &str) -> Result<SymbolFamily, std::io::Error> {
    let fd = File::open(filename)?;
    let family: SymbolFamily = serde_json::from_reader(fd)?;
    Ok(family)
}