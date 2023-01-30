#[derive(Clone)]
pub struct KanaSymbol {
    display: String,
    translations: Vec<String>
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