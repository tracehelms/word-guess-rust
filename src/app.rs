use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

pub enum Msg {
    Guess,
    SetGuess(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let word = "Animal";
        App {
            link,
            state: State::new(word)
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Guess => self.state.guess(),
            Msg::SetGuess(val) => self.state.set_guess(val.clone().remove(0)),
        }
        true
    }

    fn view(&self) -> Html {
        let value = match &self.state.guess_value {
            None => String::from(""),
            Some(v) => v.to_string(),
        };

        html! {
            <div>
                <h1>{"Hangman"}</h1>
                <h2>{"Your word: "}{self.state.guessed()}</h2>
                <p>{"Guesses left: "}{self.state.guesses_left()}</p>

                <hr />

                <div>
                    <label for="guess">{"Guess a letter:"}</label>
                    <input 
                        name="guess" 
                        value=value
                        oninput=self.link.callback(|e: InputData| Msg::SetGuess(e.value))
                        />
                </div>
                <div>
                    <button onclick=self.link.callback(|_| Msg::Guess)>{"Guess"}</button>
                </div>
            </div>
        }
    }
}

pub struct State {
    word: String,
    guesses: u32,
    guessed: Vec<char>,
    guess_value: Option<char>,
}

impl State {
    pub fn new(word: &str) -> State {
        State {
            word: word.to_string(),
            guesses: 6,
            guessed: vec![],
            guess_value: None,
        }
    }

    pub fn guesses_left(&self) -> u32 {
        self.guesses
    }

    pub fn guessed(&self) -> String {
        // self.guessed.iter().map(|c| c.to_string()).collect();

        let mut result = String::from("");

        for c in self.word.chars() {
            if self.guessed.contains(&c) {
                result.push(c);
            } else {
                result.push('_');
            }
        }

        result
    }

    pub fn set_guess(&mut self, letter: char) {
        // TODO validation
        self.guess_value = Some(letter);
    }

    pub fn guess(&mut self) {
        let guess = match self.guess_value {
            None => return (),
            Some(c) => c,
        };

        if self.word.contains(&guess.to_string()) {
            self.guessed.push(guess);
        }

        self.guesses = self.guesses - 1;
        self.guess_value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let word = "hello";
        let new_game = State::new(&word);

        assert_eq!(new_game.word, word.to_string());
        assert_eq!(new_game.guesses_left(), 6);
        assert_eq!(new_game.guessed(), "_____".to_string());
    }

    #[test]
    fn test_guess_right() {
        let mut game = State::new("hello");
        game.set_guess('e');
        game.guess();

        assert_eq!(game.guesses_left(), 5);
        assert_eq!(game.guessed(), "_e___".to_string());
        assert_eq!(game.guess_value, None);
    }

    #[test]
    fn test_guess_multiple_right() {
        let mut game = State::new("hello");
        game.set_guess('l');
        game.guess();

        assert_eq!(game.guesses_left(), 5);
        assert_eq!(game.guessed(), "__ll_".to_string());
        assert_eq!(game.guess_value, None);
    }

    #[test]
    fn test_guess_wrong() {
        let mut game = State::new("hello");
        game.set_guess('a');
        game.guess();

        assert_eq!(game.guesses_left(), 5);
        assert_eq!(game.guessed(), "_____".to_string());
        assert_eq!(game.guess_value, None);
    }

    #[test]
    fn test_set_guess() {
        let mut game = State::new("hello");
        game.set_guess('a');
        assert_eq!(game.guess_value, Some('a'));
    }
}
