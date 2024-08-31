use std::fmt;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum GameStatus {
    InProgress,
    Won,
    Lost,
}

#[derive(Debug)]
pub enum GameError {
    NotInAlphabet(char),
    WrongLength { expected: usize, actual: usize },
    GameIsOver(GameStatus),
}

#[derive(Debug)]
pub struct Game {
    pub status: GameStatus,
    pub attempts: u8,
    alphabet: HashSet<char>,
    word: Word,
    history: Vec<Word>,
}

#[derive(Clone, Debug)]
enum Letter {
    Unknown(char),
    FullMatch(char),
    PartialMatch(char),
    NoMatch(char),
}

#[derive(Debug, Default, Clone)]
pub struct Word {
    letters: Vec<Letter>,
    chars: Vec<char>,
}

impl Word {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(s: &str) -> Result<Self, GameError> {
        let chars = s.chars().collect();
        let letters = s.chars().map(|c| Letter::Unknown(c)).collect();

        Ok(Word { letters, chars })
    }
}

impl Game {
    pub fn new(alphabet: &str, word: &str) -> Result<Self, GameError> {
        let alphabet: HashSet<char> = alphabet.chars().collect();
        let word = Word::from_str(word)?;
        let status = GameStatus::InProgress;

        if let Some(c) = word.chars.iter().find(|c| !alphabet.contains(&c)) {
            return Err(GameError::NotInAlphabet(*c));
        }

        let history = vec![word.clone()];

        Ok(Game { alphabet, word, history, status, attempts: 0 })
    }

    pub fn guess_word(&mut self, guess: &str) -> Result<Word, GameError> {
        if !matches!(self.status, GameStatus::InProgress) {
            return Err(GameError::GameIsOver(self.status));
        }

        let guess_len = guess.chars().count();
        let word_len = self.word.chars.len();

        if guess_len != word_len {
            return Err(GameError::WrongLength { expected: word_len, actual: guess_len });
        }

        if let Some(c) = guess.chars().find(|c| !self.alphabet.contains(&c)) {
            return Err(GameError::NotInAlphabet(c));
        }

        let mut result = Word::new();

        for (word_char, guess_char) in self.word.chars.iter().zip(guess.chars()) {
            result.chars.push(guess_char);

            result.letters.push(
                if *word_char == guess_char {
                    Letter::FullMatch(guess_char)
                } else if self.word.chars.iter().find(|c| **c == guess_char).is_some() {
                    Letter::PartialMatch(guess_char)
                } else {
                    Letter::NoMatch(guess_char)
                }
            );
        }

        self.history.push(result.clone());
        self.attempts += 1;

        if result.letters.iter().all(|l| matches!(l, Letter::FullMatch(_))) {
            self.status = GameStatus::Won;
        } else if self.attempts >= 5 {
            self.status = GameStatus::Lost;
        }

        Ok(result)
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(_)      => write!(f, "|_|"),
            Self::FullMatch(c)    => write!(f, "[{}]", c.to_uppercase()),
            Self::PartialMatch(c) => write!(f, "({})", c.to_uppercase()),
            Self::NoMatch(c)      => write!(f, ">{}<", c.to_uppercase()),
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for l in &self.letters {
            write!(f, "{}", l)?;
        }

        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut history = self.history.iter();

        if let Some(word) = history.next() {
            write!(f, "{}", word)?;

            for word in history {
                write!(f, "\n{}", word)?;
            }
        }

        Ok(())
    }
}
