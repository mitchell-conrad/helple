pub mod solver;

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

fn load(_env: rustler::Env, _term: rustler::Term) -> bool {
    true
}

lazy_static! {
    pub static ref WORDS: Vec<String> = get_words();
}

rustler::init!(
    "Elixir.WordleCompanion.RustSolver",
    [external_calc, external_words],
    load = load
);

fn get_words() -> Vec<String> {
    include_str!("resources/words.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
}
