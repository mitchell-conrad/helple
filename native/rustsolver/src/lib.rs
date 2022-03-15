pub mod solver;
pub mod stats;

#[macro_use]
extern crate lazy_static;

#[rustler::nif]
fn external_calc(solution: &str, guesses: Vec<String>) -> Vec<usize> {
    solver::calc(&WORDS, solution, guesses)
}

#[rustler::nif]
fn external_words(solution: &str, guesses: Vec<String>) -> Vec<String> {
    solver::last_words_mr_bond(&WORDS, solution, guesses, 5)
}

#[rustler::nif]
fn external_std_dev(guess_histogram: Vec<usize>) -> f64 {
    stats::std_dev(&guess_histogram).unwrap_or(0f64)
}

#[rustler::nif]
fn external_mean(guess_histogram: Vec<usize>) -> f64 {
    stats::mean(&guess_histogram).unwrap_or(0f64)
}

#[rustler::nif]
fn external_count(guess_histogram: Vec<usize>) -> usize {
    stats::count(&guess_histogram)
}

fn load(_env: rustler::Env, _term: rustler::Term) -> bool {
    true
}

lazy_static! {
    pub static ref WORDS: Vec<String> = get_words();
}

rustler::init!(
    "Elixir.WordleCompanion.RustSolver",
    [
        external_calc,
        external_words,
        external_mean,
        external_std_dev,
        external_count
    ],
    load = load
);

fn get_words() -> Vec<String> {
    include_str!("resources/words.txt")
        .lines()
        .map(|line| line.to_string())
        .filter(|word| word.len() == 5)
        .collect()
}
