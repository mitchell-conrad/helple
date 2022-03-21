use rand::{prelude::SliceRandom, thread_rng};
use std::iter::{zip, FromIterator};

type PosTuple = (char, usize);
type PosVec = Vec<PosTuple>;
type PosSlice<'a> = &'a [PosTuple];

fn contains_any(word: &str, chars: &str) -> bool {
    chars.chars().any(|c| word.contains(c))
}

fn contains_all(word: &str, chars: &str) -> bool {
    chars.chars().all(|c| word.contains(c))
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

fn contains_at_any(word: &str, pos: PosSlice) -> bool {
    pos.iter()
        .map(|(val, pos)| contains_at(word, *val, *pos))
        .any(|b| b)
}
pub fn remaining_wordles(
    word_list: &[String],
    invalid_chars: Vec<char>,
    misplaced_positions: PosSlice,
    correct_positions: PosSlice,
) -> usize {
    let misplaced_positions_chars =
        String::from_iter(misplaced_positions.iter().map(|(val, _)| val));
    let invalid_chars = String::from_iter(invalid_chars.into_iter());

    word_list
        .iter()
        .filter(|word| !contains_any(word, &invalid_chars))
        .filter(|word| contains_all(word, &misplaced_positions_chars))
        .filter(|word| !contains_at_any(word, misplaced_positions))
        .filter(|word| contains_at_all(word, correct_positions))
        .count()
}

pub fn remaining_wordles_words(
    word_list: &[String],
    invalid_chars: Vec<char>,
    misplaced_positions: PosSlice,
    correct_positions: PosSlice,
) -> Vec<String> {
    let misplaced_positions_chars =
        String::from_iter(misplaced_positions.iter().map(|(val, _)| val));
    let invalid_chars = String::from_iter(invalid_chars.into_iter());

    word_list
        .iter()
        .filter(|word| !contains_any(word, &invalid_chars))
        .filter(|word| contains_all(word, &misplaced_positions_chars))
        .filter(|word| !contains_at_any(word, misplaced_positions))
        .filter(|word| contains_at_all(word, correct_positions))
        .cloned()
        .collect()
}

fn get_non_participating_chars(solution: &str, guess: &str) -> Vec<char> {
    // For the provided solution and guess: Returns all the characters contained
    // in guess which do not occur in solution.
    guess.chars().filter(|c| !solution.contains(*c)).collect()
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

type Truple = (Vec<char>, PosVec, PosVec);

fn get_all(solution: &str, guess: &str) -> Truple {
    (
        get_non_participating_chars(solution, guess),
        get_misplaced_chars(solution, guess),
        get_correct_chars(solution, guess),
    )
}

pub fn words(
    word_list: &[String],
    solution: &str,
    guesses: Vec<String>,
    sample_size: usize,
) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut rng = thread_rng();
    let mut t: Truple = get_all(
        &solution.to_lowercase(),
        &guesses.first().unwrap().to_lowercase(),
    );
    let first_result = remaining_wordles_words(word_list, t.0.clone(), &t.1, &t.2);
    out.push(
        first_result
            .choose_multiple(&mut rng, sample_size)
            .cloned()
            .collect(),
    );

    for g in guesses.into_iter().skip(1) {
        let mut other = get_all(&solution.to_lowercase(), &g.to_lowercase());
        t.0.append(&mut other.0);
        t.1.append(&mut other.1);
        t.2.append(&mut other.2);
        out.push(
            remaining_wordles_words(word_list, t.0.clone(), &t.1, &t.2)
                .choose_multiple(&mut rng, sample_size)
                .cloned()
                .collect(),
        );
    }
    out
}

pub fn last_words_mr_bond(
    word_list: &[String],
    solution: &str,
    guesses: Vec<String>,
    sample_size: usize,
) -> Vec<String> {
    let mut t: Truple = get_all(
        &solution.to_lowercase(),
        &guesses.first().unwrap().to_lowercase(),
    );

    for g in guesses.into_iter().skip(1) {
        let mut other = get_all(&solution.to_lowercase(), &g.to_lowercase());
        t.0.append(&mut other.0);
        t.1.append(&mut other.1);
        t.2.append(&mut other.2);
    }
    let mut rng = thread_rng();
    remaining_wordles_words(word_list, t.0.clone(), &t.1, &t.2)
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect()
}

pub fn calc(word_list: &[String], solution: &str, guesses: Vec<String>) -> Vec<usize> {
    let mut out = Vec::new();
    let mut t: Truple = get_all(
        &solution.to_lowercase(),
        &guesses.first().unwrap().to_lowercase(),
    );
    let first_result = remaining_wordles(word_list, t.0.clone(), &t.1, &t.2);
    out.push(first_result);

    for g in guesses.into_iter().skip(1) {
        let mut other = get_all(&solution.to_lowercase(), &g.to_lowercase());
        t.0.append(&mut other.0);
        t.1.append(&mut other.1);
        t.2.append(&mut other.2);
        out.push(remaining_wordles(word_list, t.0.clone(), &t.1, &t.2));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::WORDS;
    use super::*;
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;

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
    fn test_contains_any() {
        assert!(!contains_any("eeuib", "asdf"));
        assert!(contains_any("eeaib", "asdf"));
    }

    #[test]
    fn test_contains_all() {
        assert!(contains_all("asdf", "ad"));
        assert!(!contains_all("asdf", "az"));
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
            remaining_wordles(&WORDS, Vec::from_iter("tread".chars()), &vec!(), &vec!()),
            1624
        );
        assert_eq!(
            remaining_wordles(
                &WORDS,
                Vec::from_iter("treadbo".chars()),
                &vec!(('s', 4)),
                &vec!(('i', 2), ('l', 3))
            ),
            6
        );
        assert_eq!(
            remaining_wordles(
                &WORDS,
                Vec::from_iter("treadbok".chars()),
                &vec!(('s', 4)),
                &vec!(('i', 2), ('l', 3), ('s', 0))
            ),
            5
        );
    }

    #[test]
    fn test_get_non_participating_chars() {
        assert_eq!(get_non_participating_chars("asdf", "abcd"), vec!('b', 'c'));
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

        let guesses = words(
            &word_list,
            &"rupee".to_string(),
            vec!["peers".to_string()],
            5,
        );
        assert!(guesses.len() == 1);
        // The guess `peers` reveals that index 1,2 cannot be e.
        // This rules out `queue` leaving just rupee
        assert_slices_equal(&guesses.get(0).unwrap(), &vec!["rupee".to_string()]);
    }

    #[test]
    fn cheats_lul() {
        let r = remaining_wordles_words(
            &WORDS,
            Vec::from_iter("teadbils".chars()),
            &vec![('r', 1), ('u', 1), ('m', 2)],
            &vec![('o', 1)],
        );
        println!("{:?}", r);
        println!("{:?}", r.len());

        assert!(true);

        let a = words(
            &WORDS,
            "other",
            vec![
                "tread".to_string(),
                "boils".to_string(),
                "humpy".to_string(),
            ],
            5,
        );
        println!("{:?}", a);
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
}
