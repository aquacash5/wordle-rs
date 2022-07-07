use std::io::Write;

use anyhow::Result;
use console::{style, Key, Term};

include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));

fn mark_guess(answer: &String, guess: &String) -> String {
    let mut temp_answer = answer.clone();
    let mut marked: Vec<String> = guess.chars().map(|_| String::from(" ")).collect();

    // mark correct answers
    for (i, g) in guess.char_indices() {
        if answer.chars().nth(i).unwrap_or_default() == g {
            marked[i] = style(g).black().on_green().to_string();
            temp_answer.replace_range(i..=i, " ");
        }
    }

    // mark wrong and misplaced
    for (i, g) in guess.char_indices() {
        if marked[i] == " " {
            marked[i] = if temp_answer.contains(g) {
                temp_answer = temp_answer.replacen(g, " ", 1);
                style(g).black().on_yellow().to_string()
            } else {
                style(g).white().on_black().to_string()
            };
        }
    }
    marked.join("")
}

fn main() -> Result<()> {
    let mut term = Term::stdout();
    let mut guess = String::new();
    let mut answer: String;
    let mut exit = false;

    // get answer word
    term.clear_screen()?;
    term.write("Enter 5 letter secret answer: ".as_bytes())?;
    answer = term.read_secure_line()?.to_uppercase();

    // user entered bad answer word
    while answer.len() != 5 && !DICTIONARY.contains(&answer) {
        term.clear_screen()?;
        if answer.len() != 5 {
            term.write_line(&style("Answer must be 5 letters long").red().to_string())?;
        } else {
            term.write_line(&style("Answer is not a valid word").red().to_string())?;
        }
        term.write("Enter 5 letter secret answer: ".as_bytes())?;
        answer = term.read_secure_line()?.to_uppercase();
    }

    // play the game
    term.clear_screen()?;
    term.write_line("Enter 5 letter guesses:")?;
    while !exit {
        let key_pressed = term.read_key()?;
        match key_pressed {
            Key::Char(c) => {
                if c.is_ascii_alphabetic() {
                    let c = c.to_ascii_uppercase();
                    guess.push(c);
                    term.write(c.to_string().as_bytes())?;
                }
            }
            Key::Enter => {
                term.clear_line()?;
                if DICTIONARY.contains(&guess) {
                    term.write_line(&mark_guess(&answer, &guess))?;
                    exit = answer == guess;
                    guess.clear();
                }
                term.write(&guess.as_bytes())?;
            }
            Key::Backspace => {
                guess.pop();
                term.clear_chars(1)?;
            }
            _ => {}
        }
    }
    Ok(())
}
