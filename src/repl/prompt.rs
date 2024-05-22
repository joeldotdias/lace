pub struct ReplPrompt<'a> {
    pub logo: &'a str,
    pub symbol: &'a str,
    colour: PromptColour,
}

impl Default for ReplPrompt<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplPrompt<'_> {
    pub fn new() -> Self {
        Self {
            logo: LOGO,
            symbol: SYMBOL,
            colour: PromptColour::Works,
        }
    }

    pub fn colour(&self) -> &str {
        match self.colour {
            PromptColour::Works => "\x1b[92m",
            PromptColour::Error => "\x1b[91m",
        }
    }

    pub fn works(&mut self) {
        self.colour = PromptColour::Works;
    }

    pub fn errored(&mut self) {
        self.colour = PromptColour::Error;
    }
}

enum PromptColour {
    Works,
    Error,
}

const LOGO: &str = "
___        ___
.'|        .'|=|`.     .'|=|_.'   .'|=|_.'
.'  |      .'  | |  `. .'  |      .'  |  ___
|   |      |   |=|   | |   |      |   |=|_.'
|   |  ___ |   | |   | `.  |  ___ |   |  ___
|___|=|_.' |___| |___|   `.|=|_.' |___|=|_.'


";

const SYMBOL: &str = ">> ";
