use rand::{prelude::SliceRandom, thread_rng};
use std::{
    collections::{HashMap, HashSet},
    iter::{zip, FromIterator},
};
type PosVec = Vec<(char, usize)>;
type PosSlice<'a> = &'a [(char, usize)];

fn contains_any(word: &str, chars: &str) -> bool {
    chars.chars().any(|c| word.contains(c))
}

fn contains_all(word: &str, chars: &str) -> bool {
    chars.chars().all(|c| word.contains(c))
}

fn contains_at(word: &str, c: char, pos: usize) -> bool {
    if let Some(val) = word.chars().nth(pos) {
        return val == c;
    }
    false
}

fn contains_at_all(word: &str, pos: PosSlice) -> bool {
    let mut acc = true;

    for (val, pos) in pos {
        acc = acc && contains_at(word, *val, *pos);
    }
    acc
}

fn contains_at_any(word: &str, pos: PosSlice) -> bool {
    let mut acc = false;
    for (val, pos) in pos {
        acc = acc || contains_at(word, *val, *pos);
    }
    acc
}

pub fn remaining_wordles(
    word_list: &[String],
    invalid: Vec<char>,
    blue_pos: PosSlice,
    orange_pos: PosSlice,
) -> usize {
    let blue_chars = String::from_iter(blue_pos.iter().map(|(val, _)| val));
    let invalid = String::from_iter(invalid.into_iter());
    word_list
        .iter()
        .filter(|word| !contains_any(word.as_ref(), &invalid))
        .filter(|word| contains_all(word.as_ref(), &blue_chars))
        .filter(|word| !contains_at_any(word.as_ref(), blue_pos))
        .filter(|word| contains_at_all(word.as_ref(), orange_pos))
        .count()
}

pub fn remaining_wordles_words(
    word_list: &[String],
    invalid: Vec<char>,
    blue_pos: PosSlice,
    orange_pos: PosSlice,
) -> Vec<String> {
    let blue_chars = String::from_iter(blue_pos.iter().map(|(val, _)| val));
    let invalid = String::from_iter(invalid.into_iter());

    word_list
        .iter()
        .filter(|word| !contains_any(word, &invalid))
        .filter(|word| contains_all(word, &blue_chars))
        .filter(|word| !contains_at_any(word, blue_pos))
        .filter(|word| contains_at_all(word, orange_pos))
        .map(|s| s.to_string())
        .collect()
}

fn get_grey(solution: &str, guess: &str) -> Vec<char> {
    let mut out = Vec::new();
    for val in guess.chars() {
        if !solution.contains(val) {
            out.push(val);
        }
    }
    out
}

fn get_orange(solution: &str, guess: &str) -> PosVec {
    let mut out = Vec::new();
    for (i, (s, g)) in zip(solution.chars(), guess.chars()).enumerate() {
        if s == g {
            out.push((s, i));
        }
    }
    out
}

fn get_blue(solution: &str, guess: &str) -> PosVec {
    let orange_idxs: HashSet<usize> = get_orange(solution, guess)
        .into_iter()
        .map(|(_, c)| c)
        .collect();

    let mut max_blues_by_letter: HashMap<char, u8> = HashMap::new();
    // This should be the max number of each letter in the solution that we
    // should reveal as blue.
    for (idx, c) in solution.chars().enumerate() {
        if !orange_idxs.contains(&idx) && guess.contains(c) {
            *max_blues_by_letter.entry(c).or_insert(0) += 1;
        }
    }

    let mut out = Vec::new();
    for c in max_blues_by_letter.clone().keys() {
        // The letter c may exist more than once in the provided guess, so get
        // the index for all instances of c which are in the wrong location.
        let incorrect_idxs = guess
            .chars()
            .enumerate()
            .filter(|(idx, g)| g == c && !orange_idxs.contains(idx))
            .map(|(idx, _)| idx);

        for idx in incorrect_idxs {
            // Reveal up to the count of the non-correct instances of the given
            // letter in the solution.
            if max_blues_by_letter.get(c).unwrap_or(&0u8) > &0u8 {
                *max_blues_by_letter.entry(*c).or_insert(0u8) -= 1;
                out.push((*c, idx));
            }
        }
    }
    out
}

type Truple = (Vec<char>, PosVec, PosVec);

fn get_all(solution: &str, guess: &str) -> Truple {
    (
        get_grey(solution, guess),
        get_blue(solution, guess),
        get_orange(solution, guess),
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
            vec!(1625, 6, 3, 2)
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
            1625
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
    fn test_grey() {
        assert_eq!(get_grey("asdf", "abcd"), vec!('b', 'c'));
    }

    #[test]
    fn test_orange() {
        assert_eq!(get_orange("abcd", "abxy"), vec!(('a', 0), ('b', 1)));
        assert_eq!(get_orange("zxym", "abxy"), vec!());
    }

    #[test]
    fn test_blue() {
        // If a letter is in a solution twice (in this case `a`). The letter
        // which is in the correct location shall be ignored.
        assert_slices_equal(
            &get_blue("alarm", "drama"),
            &vec![('r', 1), ('m', 3), ('a', 4)],
        );

        // If a letter appears in the solution twice (in this case `l`)
        // Only the first incorrect occurence will be returned as blue.
        // reading from left to right)
        assert_slices_equal(&get_blue("swill", "lolly"), &vec![('l', 0)]);
        // If there is more than 1 instance of a guess letter. whilst only there
        // is only 1 location in the solution, reveal the letter as orange
        // as precedence over blue.
        assert_slices_equal(&get_blue("dance", "nanas"), &vec![]);

        // If there are multiple instances of the same letter in a guess, and
        // there is only a singular instance of that letter in the solution.
        // Reveal the first (reading left to right) instance in the guess as blue.
        assert_slices_equal(&get_blue("swirl", "lolly"), &vec![('l', 0)]);
        assert_slices_equal(&get_blue("hoard", "nanas"), &vec![('a', 1)]);
        assert_slices_equal(&get_blue("swill", "tease"), &vec![('s', 3)]);

        assert_slices_equal(&get_blue("swill", "boils"), &vec![('s', 4)]);
        assert_slices_equal(
            &get_blue("caulk", "aloud"),
            &vec![('a', 0), ('l', 1), ('u', 3)],
        );

        assert_slices_equal(
            &get_blue("abbba", "babab"),
            &vec![('b', 0), ('a', 1), ('a', 3), ('b', 4)],
        );
        assert_slices_equal(
            &get_blue("ababa", "babab"),
            &vec![('b', 0), ('a', 1), ('b', 2), ('a', 3)],
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

    fn assert_slices_equal<T: Eq + Hash>(a: &[T], b: &[T]) {
        let a_len = a.len();
        let b_len = b.len();
        if a_len != b_len {
            assert!(false, "a and b have differing lengths")
        } else {
            let a_set: HashSet<&T> = HashSet::from_iter(a.into_iter());
            let b_set: HashSet<&T> = HashSet::from_iter(b.into_iter());
            assert!(a_set == b_set, "a and b have differing contents")
        }
    }
}