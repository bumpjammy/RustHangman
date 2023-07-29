const LIVES: u8 = 6;

struct GameData {
    secret_word               : String,
    discovered_letters        : Vec<char>,
    lives                     : u8,
}

fn get_random_word() -> String {
    let word_list = [
        "hello",
        "world",
        "rust",
        "programming",
        "computer",
        "science",
        "mathematics",
        "physics",
        "coding",
        "software",
    ];
    let index = rand::random::<usize>() % word_list.len();
    return String::from(word_list[index]);
}

fn get_hangman_art(game_data: &GameData) -> String {
    let index = LIVES - game_data.lives;
    let hangman_art = [
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
    ]; // Credit to https://gist.github.com/chrishorton/8510732aa9a80a03c829b09f12e20d9c

    return String::from(hangman_art[index as usize]);
}

fn print_game_state(game_data: &GameData) {
    println!("");
    println!("---------------------------------");
    println!("");
    println!("{}", get_hangman_art(&game_data));
    print!("Word: ");
    for c in game_data.secret_word.chars() {
        if game_data.discovered_letters.contains(&c) {
            print!("{}", c);
        } else {
            print!("_");
        }
    }
    println!("\nGuess a letter: ");
}

fn read_guess() -> char {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    return guess.chars().next().unwrap();
}

fn is_guess_correct(guess: char, game_data: &GameData) -> Result<bool, &'static str> {
    if !guess.is_alphabetic() {
        return Err("Please enter a letter!");
    }
    if game_data.discovered_letters.contains(&guess) {
        return Err("You've already guessed that letter!");
    }
    return Ok(game_data.secret_word.contains(guess));
}

fn is_solved(game_data: &GameData) -> bool {
    for c in game_data.secret_word.chars() {
        if !game_data.discovered_letters.contains(&c) {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut game_data = GameData {
        secret_word: get_random_word(),
        discovered_letters: vec![],
        lives: LIVES,
    };

    while game_data.lives > 0 {
        print_game_state(&game_data);
        let guess = read_guess().to_ascii_lowercase();

        let result = is_guess_correct(guess.to_ascii_lowercase(), &game_data);
        match result {
            Ok(true) => {
                println!("Correct!");
                game_data.discovered_letters.push(guess);
            },
            Ok(false) => {
                println!("Wrong!");
                game_data.lives -= 1;
            }
            Err(e) => println!("{}", e),
        }

        if is_solved(&game_data) {
            println!("You win!"); // Gamer moment
            return;
        }
    }
    println!("You lose!"); // Not a gamer moment
    println!("{}", get_hangman_art(&game_data));
}