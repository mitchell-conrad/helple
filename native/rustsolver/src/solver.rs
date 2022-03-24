use rand::{prelude::SliceRandom, thread_rng};
use std::collections::HashMap;
use std::iter::zip;
use std::ops::{Add, AddAssign};

type PosTuple = (char, usize);
type PosVec = Vec<PosTuple>;
type PosSlice<'a> = &'a [PosTuple];

struct GuessResult {
    max_participating: HashMap<char, usize>,
    misplaced: PosVec,
    correct: PosVec,
}

impl AddAssign for GuessResult {
    fn add_assign(&mut self, other: Self) {
        self.max_participating.extend(other.max_participating);
        self.misplaced.extend(other.misplaced);
        self.correct.extend(other.correct);
    }
}
impl Add for GuessResult {
    type Output = GuessResult;
    fn add(self, rhs: Self) -> Self::Output {
        let mut guess = self;
        guess += rhs;
        guess
    }
}

fn contains_all(word: &str, chars: &[char]) -> bool {
    chars.iter().all(|c| word.contains(*c))
}

fn contains_at(word: &str, c: char, pos: usize) -> bool {
    match word.chars().nth(pos) {
        Some(val) => val == c,
        None => false,
    }
}

fn contains_at_all(word: &str, pos: PosSlice) -> bool {
    pos.iter()
        .map(|(val, pos)| contains_at(word, *val, *pos))
        .all(|b| b)
}

fn get_letter_counts(word: &str) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in word.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn contains_no_more_than(word: &str, max_counts: &HashMap<char, usize>) -> bool {
    let word_counts = get_letter_counts(word);

    !max_counts.iter().any(|(c, max_count)| {
        // Check to see if the word has more than the max number of each letter
        let word_count_opt = word_counts.get(c);

        if let Some(word_count) = word_count_opt {
            // Check if the word contains more than allowed max of the char
            word_count > max_count
        } else {
            // If the word doesn't contain the letter at all, we're okay.
            false
        }
    })
}

fn contains_at_any(word: &str, pos: PosSlice) -> bool {
    pos.iter()
        .map(|(val, pos)| contains_at(word, *val, *pos))
        .any(|b| b)
}

fn remaining_wordles_words(word_list: &[String], t: &GuessResult) -> Vec<String> {
    let misplaced_positions_chars: Vec<char> = t.misplaced.iter().map(|(val, _)| *val).collect();

    word_list
        .iter()
        .filter(|word| contains_no_more_than(word, &t.max_participating))
        .filter(|word| contains_all(word, &misplaced_positions_chars))
        .filter(|word| !contains_at_any(word, &t.misplaced))
        .filter(|word| contains_at_all(word, &t.correct))
        .cloned()
        .collect()
}

fn get_max_char_counts(solution: &str, guess: &str) -> HashMap<char, usize> {
    // If the guess has more instances of a character than the solution has, we
    // can prune any word which has more instances of the character than the
    // solution. This function produces a map which contains this list.

    let solution_counts = get_letter_counts(solution);
    let guess_counts = get_letter_counts(guess);

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for (c, guess_count) in guess_counts.iter() {
        let solution_count_opt = solution_counts.get(c);

        if let Some(solution_count) = solution_count_opt {
            if *guess_count > *solution_count {
                char_counts.insert(*c, *solution_count);
            }
        } else {
            char_counts.insert(*c, 0);
        }
    }
    char_counts
}

fn get_correct_chars(solution: &str, guess: &str) -> PosVec {
    // For the provided solution and guess: Returns all the instances where a
    // character in the guess is in the correct position.

    zip(solution.chars(), guess.chars())
        .enumerate()
        // Filter for chars that are in the same position in both guess and solution
        .filter(|(_, (solution_char, guess_char))| solution_char == guess_char)
        // Map to pos tuple
        .map(|(index, (solution_char, _))| (solution_char, index))
        .collect()
}

fn get_misplaced_chars(solution: &str, guess: &str) -> PosVec {
    // For the provided solution and guess: Returns all the instances where a
    // character in the guess occurs in the solution, but is in the incorrect
    // position.

    zip(solution.chars(), guess.chars())
        .enumerate()
        // Filter for chars that both solution and guess contain but aren't
        //  in the same position
        .filter(|(_, (solution_char, guess_char))| {
            solution_char != guess_char && solution.contains(*guess_char)
        })
        // Map to PosTuple
        .map(|(index, (_, guess_char))| (guess_char, index))
        .collect()
}

fn get_all(solution: &str, guess: &str) -> GuessResult {
    GuessResult {
        max_participating: get_max_char_counts(solution, guess),
        misplaced: get_misplaced_chars(solution, guess),
        correct: get_correct_chars(solution, guess),
    }
}

pub fn last_words_mr_bond(
    word_list: &[String],
    solution: &str,
    guesses: Vec<String>,
    sample_size: usize,
) -> Vec<String> {
    // Flatten the GuessResults into a singular GuessResult.
    let guess_result: GuessResult = guesses
        .into_iter()
        .map(|guess| get_all(&solution.to_lowercase(), &guess.to_lowercase()))
        .fold(
            GuessResult {
                max_participating: HashMap::new(),
                misplaced: vec![],
                correct: vec![],
            },
            |a, b| a + b,
        );
    remaining_wordles_words(word_list, &guess_result)
        .choose_multiple(&mut thread_rng(), sample_size)
        .cloned()
        .collect()
}

pub fn calc(word_list: &[String], solution: &str, guesses: Vec<String>) -> Vec<usize> {
    // Take a copy of the words list
    let mut word_list: Vec<String> = word_list.to_vec();

    guesses
        .into_iter()
        .map(|guess| get_all(&solution.to_lowercase(), &guess.to_lowercase()))
        .map(|guess_result| {
            // Each guess enables us to prune down word_list.
            word_list = remaining_wordles_words(word_list.as_slice(), &guess_result);
            word_list.len()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::WORDS;
    use super::*;
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::iter::FromIterator;

    #[test]
    fn test_calc() {
        assert_eq!(
            calc(
                &WORDS,
                "swill",
                vec!(
                    "tread".to_string(),
                    "boils".to_string(),
                    "skill".to_string(),
                    "spill".to_string()
                )
            ),
            vec!(1624, 6, 3, 2)
        );

        assert_eq!(
            calc(
                &WORDS,
                "shake",
                vec!(
                    "suite".to_string(),
                    "snare".to_string(),
                    "spade".to_string(),
                    "shame".to_string(),
                    "shale".to_string()
                )
            ),
            vec!(96, 22, 14, 3, 2)
        );

        assert_eq!(
            calc(
                &WORDS,
                "shake",
                vec!(
                    "suitE".to_string(),
                    "snAre".to_string(),
                    "spade".to_string(),
                    "shame".to_string(),
                    "shale".to_string()
                )
            ),
            vec!(96, 22, 14, 3, 2)
        );
    }

    #[test]
    fn test_get_words() {
        assert_eq!("aahed", WORDS.first().unwrap());
    }

    #[test]
    fn test_contains_no_more_than() {
        // eerie contains no more than 3 e
        assert!(contains_no_more_than("eerie", &HashMap::from([('e', 3)])));
        // eerie contains no more than 2 r
        assert!(contains_no_more_than("eerie", &HashMap::from([('r', 2)])));
        // eerie contains no more than 1 q
        assert!(contains_no_more_than("eerie", &HashMap::from([('q', 1)])));
        // eerie contains more than 2 e
        assert!(!contains_no_more_than("eerie", &HashMap::from([('e', 2)])));
        // eerie contains more than 1 e
        assert!(!contains_no_more_than("eerie", &HashMap::from([('e', 1)])));
        // eerie contains more than 0 r
        assert!(!contains_no_more_than("eerie", &HashMap::from([('r', 0)])));
    }

    #[test]
    fn test_get_max_char_counts() {
        // The guess doesn't reveal more e's than the solution, so we can't
        // infer a max count
        assert_eq!(get_max_char_counts("eerie", "e"), HashMap::from([]));
        // The guess still doesn't reveal more e's than the solution, so we
        // can't infer a max count
        assert_eq!(get_max_char_counts("eerie", "ee"), HashMap::from([]));

        // The guess still doesn't reveal more e's than the solution, so we
        // can't infer a max count
        assert_eq!(get_max_char_counts("eerie", "eee"), HashMap::from([]));

        // The guess reveals more e's than the solution, so we can infer that
        // there is at most 3 e's
        assert_eq!(
            get_max_char_counts("eerie", "eeee"),
            HashMap::from([('e', 3)])
        );

        // The guess reveals more e's than the solution, so we can infer that
        // there is at most 3 e's.
        assert_eq!(
            get_max_char_counts("poops", "pppooos"),
            HashMap::from([('p', 2), ('o', 2)])
        );
    }

    #[test]
    fn test_contains_all() {
        assert!(contains_all("asdf", &str2vec("ad")));
        assert!(!contains_all("asdf", &str2vec("az")));
    }

    #[test]
    fn test_contains_at() {
        assert!(contains_at("adsf", 'a', 0));
        assert!(!contains_at("adsf", 'a', 1));
        assert!(!contains_at("adsf", 'a', 10));
    }

    #[test]
    fn test_contains_at_all() {
        assert!(contains_at_all("asdf", &vec!(('a', 0), ('s', 1), ('d', 2))));
        assert!(!contains_at_all(
            "asdf",
            &vec!(('a', 0), ('s', 1), ('a', 2))
        ));
        assert!(!contains_at_all(
            "asdf",
            &vec!(('a', 0), ('s', 1), ('z', 2))
        ));
        assert!(!contains_at_all(
            "asdf",
            &vec!(('a', 0), ('s', 1), ('a', 9))
        ));
    }

    #[test]
    fn test_contains_at_any() {
        assert!(contains_at_any("asdf", &vec!(('a', 0))));
        assert!(contains_at_any("asdf", &vec!(('z', 0), ('s', 1))));
        assert!(!contains_at_any("asdf", &vec!(('z', 0))));
        assert!(!contains_at_any("asdf", &vec!(('z', 9))));
        assert!(!contains_at_any("asdf", &vec!(('a', 9))));
    }

    #[test]
    fn test_remaining_wordles() {
        assert_eq!(
            remaining_wordles_words(
                &WORDS,
                &GuessResult {
                    max_participating: non_participating_count("tread"),
                    misplaced: vec!(),
                    correct: vec!()
                }
            )
            .len(),
            1624
        );
        assert_eq!(
            remaining_wordles_words(
                &WORDS,
                &GuessResult {
                    max_participating: non_participating_count("treadbo"),
                    misplaced: vec!(('s', 4)),
                    correct: vec!(('i', 2), ('l', 3))
                }
            )
            .len(),
            6
        );
        assert_eq!(
            remaining_wordles_words(
                &WORDS,
                &GuessResult {
                    max_participating: non_participating_count("treadbok"),
                    misplaced: vec!(('s', 4)),
                    correct: vec!(('i', 2), ('l', 3), ('s', 0))
                }
            )
            .len(),
            5
        )
    }

    #[test]
    fn test_last_words_mr_bond() {
        let v: Vec<String> = vec!["purge".to_string(), "puree".to_string()];

        assert_eq!(
            last_words_mr_bond(&v, "purge", vec!("pence".to_string()), 5),
            vec!("purge")
        );
    }

    #[test]
    fn test_get_correct_chars() {
        assert_eq!(get_correct_chars("abcd", "abxy"), vec!(('a', 0), ('b', 1)));
        assert_eq!(get_correct_chars("zxym", "abxy"), vec!());
    }

    #[test]
    fn test_get_misplaced_chars() {
        assert_slices_equal(
            &get_misplaced_chars("alarm", "drama"),
            &vec![('r', 1), ('m', 3), ('a', 4)],
        );

        assert_slices_equal(&get_misplaced_chars("smelt", "smell"), &vec![('l', 4)]);

        assert_slices_equal(
            &get_misplaced_chars("swill", "lolly"),
            &vec![('l', 0), ('l', 2)],
        );

        assert_slices_equal(
            &get_misplaced_chars("dance", "nanas"),
            &vec![('a', 3), ('n', 0)],
        );

        // If there are multiple instances of the same letter in a guess
        // Reveal all the incorrect locations as misplaced.
        assert_slices_equal(
            &get_misplaced_chars("swirl", "lolly"),
            &vec![('l', 0), ('l', 2), ('l', 3)],
        );

        assert_slices_equal(
            &get_misplaced_chars("hoard", "nanas"),
            &vec![('a', 1), ('a', 3)],
        );

        assert_slices_equal(&get_misplaced_chars("swill", "tease"), &vec![('s', 3)]);

        assert_slices_equal(&get_misplaced_chars("swill", "boils"), &vec![('s', 4)]);

        assert_slices_equal(
            &get_misplaced_chars("caulk", "aloud"),
            &vec![('a', 0), ('l', 1), ('u', 3)],
        );

        assert_slices_equal(
            &get_misplaced_chars("abbba", "babab"),
            &vec![('b', 0), ('a', 1), ('a', 3), ('b', 4)],
        );

        assert_slices_equal(
            &get_misplaced_chars("ababa", "babab"),
            &vec![('b', 0), ('a', 1), ('b', 2), ('a', 3), ('b', 4)],
        );
    }

    #[test]
    fn test_double_letter_strangeness() {
        let word_list = vec![
            "peers".to_string(),
            "queue".to_string(),
            "rupee".to_string(),
        ];

        let guesses = last_words_mr_bond(
            &word_list,
            &"rupee".to_string(),
            vec!["peers".to_string()],
            5,
        );
        assert!(guesses.len() == 1);
        // The guess `peers` reveals that index 1,2 cannot be e.
        // This rules out `queue` leaving just rupee
        assert_slices_equal(&guesses, &vec!["rupee".to_string()]);
    }

    fn assert_slices_equal<T: Eq + Hash + Debug>(a: &[T], b: &[T]) {
        let a_len = a.len();
        let b_len = b.len();
        if a_len != b_len {
            assert!(false, "A {:?} B: {:?} have differing lengths", a, b);
        } else {
            let a_set: HashSet<&T> = HashSet::from_iter(a.into_iter());
            let b_set: HashSet<&T> = HashSet::from_iter(b.into_iter());
            assert!(
                a_set == b_set,
                "A {:?} B: {:?} have differing contents",
                a,
                b
            );
        }
    }

    fn str2vec(str: &str) -> Vec<char> {
        Vec::from_iter(str.chars())
    }

    fn non_participating_count(str: &str) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();

        for c in str.chars() {
            counts.insert(c, 0);
        }
        counts
    }
}
