use crate::dictionary::get_random_word;

const NUM_GUESSES: u32 = 10;

pub struct Game {
    pub error_message: String,
    pub game_result: Option<GameResult>,
    pub guess_value: Option<char>,
    pub guessed: Vec<char>,
    pub guesses_left: u32,
    word: String,
}

pub enum GameResult {
    Won,
    Lost,
}

impl Game {
    pub fn new() -> Game {
        let word = get_random_word();

        Game {
            error_message: String::from(""),
            game_result: None,
            guess_value: None,
            guessed: vec![],
            guesses_left: NUM_GUESSES,
            word,
        }
    }

    #[allow(dead_code)]
    fn set_word<'a>(&mut self, word: &'a str) -> &'a str {
        match self.guessed.len() {
            0 => {
                self.word = word.to_string();
                &word
            }
            _ => panic!("Can't change word during a game"),
        }
    }

    pub fn word_with_guesses(&self) -> String {
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

    pub fn set_guess(&mut self, input: String) {
        let lowercased = input.to_lowercase();
        let letter = match lowercased.chars().next() {
            Some(c) => Some(c),
            _ => None,
        };
        self.guess_value = letter;
    }

    pub fn guess(&mut self) {
        self.error_message = String::from("");

        match self.guess_value {
            None => return,
            Some(c) if self.guessed.contains(&c) => {
                self.error_message = String::from("You can't guess the same letter twice.");
            }
            Some(c) if self.word.contains(&c.to_string()) => self.guessed.push(c),
            Some(c) if !c.is_alphabetic() => {
                self.error_message = String::from("Your guess has to be a letter.")
            }
            Some(c) => {
                self.guessed.push(c);
                self.guesses_left -= 1;
            }
        };

        self.guess_value = None;
        self.is_game_over_yet();
    }

    fn is_game_over_yet(&mut self) {
        if self.word_with_guesses() == self.word {
            self.game_result = Some(GameResult::Won);
        } else if self.guesses_left == 0 {
            self.game_result = Some(GameResult::Lost);
        }
    }

    pub fn game_over_message(&self) -> String {
        match &self.game_result {
            Some(GameResult::Won) => String::from("You won!"),
            Some(GameResult::Lost) => format!("You lost. The word was: {}", self.word),
            None => String::from(""),
        }
    }

    pub fn already_guessed(&self) -> String {
        let mut already_guessed = self
            .guessed
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        already_guessed.sort();
        already_guessed.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let mut new_game = Game::new();
        let word = new_game.set_word("hello");

        assert_eq!(new_game.word, word.to_string());
        assert_eq!(new_game.guesses_left, NUM_GUESSES);
        assert_eq!(new_game.word_with_guesses(), "_____".to_string());
    }

    #[test]
    #[should_panic(expected = "Can't change word during a game")]
    fn test_cant_change_word_during_game() {
        let mut game = Game::new();
        game.set_guess("a".to_string());
        game.guess();

        game.set_word("nope");
    }

    #[test]
    fn test_guess_single_right() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("e".to_string());
        game.guess();

        assert_eq!(game.guesses_left, NUM_GUESSES);
        assert_eq!(game.word_with_guesses(), "_e___".to_string());
        assert_eq!(game.guess_value, None);
    }

    #[test]
    fn test_correct_guess_doesnt_decrement_guesses() {
        let mut game = Game::new();
        assert_eq!(game.guesses_left, NUM_GUESSES);
        game.set_word("hello");
        game.set_guess("e".to_string());
        game.guess();

        assert_eq!(game.guesses_left, NUM_GUESSES);
    }

    #[test]
    fn test_guess_multiple_right() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("l".to_string());
        game.guess();

        assert_eq!(game.guesses_left, NUM_GUESSES);
        assert_eq!(game.word_with_guesses(), "__ll_".to_string());
        assert_eq!(game.guess_value, None);

        let mut game = Game::new();
        game.set_word("animal");
        game.set_guess("a".to_string());
        game.guess();

        assert_eq!(game.word_with_guesses(), "a___a_".to_string());
    }

    #[test]
    fn test_guess_wrong() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("a".to_string());
        game.guess();

        assert_eq!(game.guesses_left, NUM_GUESSES - 1);
        assert_eq!(game.word_with_guesses(), "_____".to_string());
        assert_eq!(game.guess_value, None);
    }

    #[test]
    fn test_set_guess() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("a".to_string());
        assert_eq!(game.guess_value, Some('a'));
    }

    #[test]
    fn test_multiple_letter_guess() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("abc".to_string());
        assert_eq!(game.guess_value, Some('a'));
    }

    #[test]
    fn test_empty_guess() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("".to_string());
        assert_eq!(game.guess_value, None);
    }

    #[test]
    fn test_cant_guess_same_letter_twice() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("a".to_string());
        game.guess();
        assert_eq!(game.guesses_left, NUM_GUESSES - 1);

        game.set_guess("a".to_string());
        game.guess();
        assert_eq!(game.guesses_left, NUM_GUESSES - 1);
        assert_eq!(game.error_message, "You can't guess the same letter twice.");
    }

    #[test]
    fn test_guess_has_to_be_a_letter() {
        let mut game = Game::new();
        game.set_word("hello");
        let invalid_guesses = ["#", "2", " ", "👋"];

        for guess in invalid_guesses.iter() {
            game.set_guess(guess.to_string());
            game.guess();
            assert_eq!(game.error_message, "Your guess has to be a letter.");
        }
    }

    #[test]
    fn test_word_is_random_each_time() {
        let mut used_words = vec![];

        for _ in 1..10 {
            let new_game = Game::new();
            assert_eq!(used_words.contains(&new_game.word), false);
            used_words.push(new_game.word.clone());
        }
    }

    #[test]
    fn test_guess_can_be_capital_letters() {
        let mut game = Game::new();
        game.set_word("hello");
        game.set_guess("H".to_string());
        game.guess();

        assert_eq!(game.guesses_left, NUM_GUESSES);
        assert_eq!(game.word_with_guesses(), "h____".to_string());
    }

    #[test]
    fn test_won_game() {
        let mut game = Game::new();
        game.set_word("hi");
        game.set_guess("h".to_string());
        game.guess();
        game.set_guess("i".to_string());
        game.guess();

        assert_eq!(game.game_over_message(), "You won!".to_string());
    }

    #[test]
    fn test_lost_game() {
        let mut game = Game::new();
        game.set_word("xyz");
        for letter in 'a'..'n' {
            game.set_guess(letter.to_string());
            game.guess();
        }

        assert_eq!(
            game.game_over_message(),
            "You lost. The word was: xyz".to_string()
        );
    }

    #[test]
    fn test_already_guessed() {
        let mut game = Game::new();
        for guess in ['c', 'a', 'b'].iter() {
            game.set_guess(guess.to_string());
            game.guess();
        }
        assert_eq!(game.already_guessed(), "abc");
    }
}
