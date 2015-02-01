use std::cmp::Ordering;

fn main() {
    let secret = std::rand::random::<u8>() % 100 + 1;

    // println!("the secret number is {}", secret);
    
    loop {
        let cmpd = cmp(take_guess(), secret);

        match cmpd {
            Ordering::Less => {
                println!("less");
            },
            Ordering::Greater => {
                println!("greater");
            },
            Ordering::Equal => {
                println!("bingo !");
                break
            },
        }
    }
    
}

fn take_guess() -> u8 {
    loop {
        print!("guess a number between 1 and 100: ");

        let input = std::io::stdin().read_line().ok().expect("shouldn't fail");
        match input.trim().parse::<u8>() {
            Some(q) => return q,
            None => {
                println!("Please input a number!");
            }
        };    
    }
}

fn cmp(a: u8, b: u8) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
