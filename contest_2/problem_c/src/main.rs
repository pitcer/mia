use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let mut lines = stdin.lines();

    let problem_count = lines.next().ok_or("Missing problem count line")??;
    let mut problem_count = split_parse_iter::<usize>(&problem_count);
    let problem_count = problem_count.next().ok_or("Missing problem count")??;
    debug_assert!((1..=1_000).contains(&problem_count));

    let mut counter = 0;
    for line in lines {
        let password = line?;
        debug_assert!((1..=200).contains(&password.len()));
        solve_problem(password)?;
        counter += 1;
    }
    debug_assert_eq!(counter, problem_count);
    Ok(())
}

fn solve_problem(password: String) -> Result<()> {
    let problem = Keyboard::new(password);
    let result = problem.solve()?;
    if let Some(password) = result {
        println!("YES");
        println!("{password}");
    } else {
        println!("NO")
    }
    Ok(())
}

fn split_parse_iter<T>(line: &str) -> impl Iterator<Item = Result<T, T::Err>> + '_
where
    T: FromStr,
    T::Err: std::error::Error,
{
    line.split_whitespace().map(|entry| entry.parse::<T>())
}

struct Keyboard {
    password: String,
}

impl Keyboard {
    pub fn new(password: String) -> Self {
        Self { password }
    }
}

impl Problem<Option<String>> for Keyboard {
    fn from_reader(_reader: impl BufRead) -> Result<Self> {
        unimplemented!()
    }

    fn solve(self) -> Result<Option<String>> {
        let mut password = self.password.bytes();
        let first_letter = password.next().ok_or("we assumed that |password| >= 1")?;
        let mut assembler = KeyboardAssembler::new(first_letter);
        for letter in password {
            if let AssemblyResult::Impossible = assembler.insert(letter) {
                return Ok(None);
            }
        }
        let unused_letters = assembler.used_letters.all_unset();
        let mut layout = assembler.into_layout();
        layout.push_str(&unused_letters);
        Ok(Some(layout))
    }
}

type Letter = u8;

struct KeyboardAssembler {
    used_letters: LetterSet,
    layout: VecDeque<Letter>,
    position: usize,
}

enum AssemblyResult {
    Possible,
    Impossible,
}

impl KeyboardAssembler {
    fn new(first_letter: Letter) -> KeyboardAssembler {
        let mut builder = Self {
            used_letters: LetterSet::new(),
            layout: VecDeque::with_capacity(26),
            position: 0,
        };
        builder.used_letters.set(first_letter);
        builder.layout.push_back(first_letter);
        builder
    }

    fn into_layout(self) -> String {
        let layout = Vec::from(self.layout);
        // SAFETY: bytes were a valid utf8 and they are modified nowhere.
        unsafe { String::from_utf8_unchecked(layout) }
    }

    fn insert(&mut self, letter: Letter) -> AssemblyResult {
        if self.used_letters.is_set(letter) {
            self.insert_present(letter)
        } else {
            self.insert_new(letter)
        }
    }

    fn insert_new(&mut self, letter: Letter) -> AssemblyResult {
        if self.position == 0 {
            // we can insert a new letter at the beginning of the keyboard
            self.layout.push_front(letter);
            self.used_letters.set(letter);
            AssemblyResult::Possible
        } else if self.position + 1 == self.layout.len() {
            // we can insert a new letter at the end of the keyboard
            self.layout.push_back(letter);
            self.used_letters.set(letter);
            self.position += 1;
            AssemblyResult::Possible
        } else if self.position >= self.layout.len() {
            unreachable!("position should never be larger than keyboard size")
        } else {
            // we cannot insert a new letter to keyboard
            AssemblyResult::Impossible
        }
    }

    fn insert_present(&mut self, letter: Letter) -> AssemblyResult {
        if self.position == 0 {
            self.check_next(letter)
        } else if self.position + 1 == self.layout.len() {
            self.check_previous(letter)
        } else if self.position >= self.layout.len() {
            unreachable!("position should never be larger than keyboard size")
        } else {
            let result = self.check_next(letter);
            if let AssemblyResult::Impossible = result {
                self.check_previous(letter)
            } else {
                result
            }
        }
    }

    fn check_next(&mut self, letter: Letter) -> AssemblyResult {
        let next_letter = self.layout.get(self.position + 1);
        match next_letter {
            Some(next_letter) if *next_letter == letter => {
                self.position += 1;
                AssemblyResult::Possible
            }
            _ => AssemblyResult::Impossible,
        }
    }

    fn check_previous(&mut self, letter: Letter) -> AssemblyResult {
        let previous_letter = self.layout.get(self.position - 1);
        match previous_letter {
            Some(previous_letter) if *previous_letter == letter => {
                self.position -= 1;
                AssemblyResult::Possible
            }
            _ => AssemblyResult::Impossible,
        }
    }
}

struct LetterSet {
    bits: u32,
}

impl LetterSet {
    fn new() -> Self {
        Self { bits: 0u32 }
    }

    fn set(&mut self, letter: Letter) {
        let index = Self::letter_to_index(letter);
        self.bits |= 1u32 << index;
    }

    fn is_set(&self, letter: Letter) -> bool {
        let index = Self::letter_to_index(letter);
        self.is_index_set(index)
    }

    fn is_index_set(&self, index: u8) -> bool {
        ((self.bits >> index) & 1u32) == 1u32
    }

    fn all_unset(&self) -> String {
        ('a'..='z')
            .enumerate()
            .filter(|(index, _)| !self.is_index_set(*index as u8))
            .map(|(_, letter)| letter)
            .collect()
    }

    fn letter_to_index(letter: Letter) -> u8 {
        letter - b'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Keyboard {
            password: "ababa".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(Some("bacdefghijklmnopqrstuvwxyz".to_owned()), actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Keyboard {
            password: "codedoca".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(Some("edocabfghijklmnpqrstuvwxyz".to_owned()), actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = Keyboard {
            password: "abcda".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(None, actual);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let problem = Keyboard {
            password: "zxzytyz".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(Some("xzytabcdefghijklmnopqrsuvw".to_owned()), actual);
        Ok(())
    }

    #[test]
    fn test_example_5() -> Result<()> {
        let problem = Keyboard {
            password: "abcdefghijklmnopqrstuvwxyza".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(None, actual);
        Ok(())
    }
}
