// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashSet;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print_so_far_word(so_far_word : &Vec<char>) {
    print!("The word so far is ");
    for ch in so_far_word {
        print!("{}",ch);
    }
    println!("");
}

fn print_guessed_word(gussed_word : &HashSet<char>) {
    print!("You have guessed the following letters: ");
    for ch in gussed_word {
        print!("{}",ch);
    }
    println!("");
}

fn main() {
    println!("Welcome to CS110L Hangman!");
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut remain_guess: u32 = NUM_INCORRECT_GUESSES;
    let mut so_far_word: Vec<char> = Vec::new();
    let mut gussed_word = HashSet::new();
    let mut no_gussed_word = HashSet::new();
    for i in 0..secret_word_chars.len() {
        so_far_word.push('-');
        no_gussed_word.insert(secret_word_chars[i]);
    }
    loop {
        print_so_far_word(&so_far_word);
        print_guessed_word(&gussed_word);
        println!("You have {} guesses left", remain_guess);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess_char : Vec<char> = guess.chars().collect();
        if no_gussed_word.contains(&guess_char[0]){
            no_gussed_word.remove(&guess_char[0]);
            gussed_word.insert(guess_char[0]);
            for i in 0..secret_word_chars.len() {
                if secret_word_chars[i] == guess_char[0] {
                    so_far_word[i] = guess_char[0];
                }
            }
        }else{
            remain_guess -= 1;
            println!("Sorry, that letter is not in the word");
        }
        println!("");
        if remain_guess == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        if no_gussed_word.is_empty() {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }
    }
}
