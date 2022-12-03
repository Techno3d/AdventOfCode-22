pub fn day2(encrypted_message: String) {
    let mut score = 0;
    let mut real_score = 0;
    for line in encrypted_message.trim().split('\n') {
        let chosen = line.split(' ').map(|x| {
            convert(&x)
        }).collect::<Vec<i32>>();
        score += RPS_value(&chosen);
        real_score += real_RPS_value(&chosen);

    }
    println!("Best plan score is: {}", score);
    println!("The real best plan score is: {}", real_score);
    /*
    println!("{:?}", encrypted_message.split('\n')
        .map(|x| x.split(' ')
             .map(|y| convert(&y)))
        .collect::<Vec<&str>>());
    */
}

fn convert(letter: &str) -> i32 {
    if letter == "A" || letter == "X" {
        return 1;
    } else if letter == "B" || letter == "Y" {
        return 2;
    } else {
        return 3;
    }
}

fn RPS_value(plays: &Vec<i32>) -> i32 {
    const ROCK: i32 = 1;
    const PAPER: i32 = 2;
    const SCISSORS: i32 = 3;

    let other = plays[0];
    let player = plays[1];
    let mut is_player_win = true;
    let mut is_tie = false;
    if other == ROCK && player == SCISSORS {
        is_player_win = false;
    } else if other == PAPER && player == ROCK {
        is_player_win = false;
    } else if other == SCISSORS && player == PAPER {
        is_player_win = false;
    } else if other == player {
        is_tie = true;
    }

    let mut score = player;
    if is_tie {
        score += 3
    } else if is_player_win {
        score += 6
    }

    return score;
}

fn real_RPS_value(plays: &Vec<i32>) -> i32 {
    const ROCK: i32 = 1;
    const PAPER: i32 = 2;
    const SCISSORS: i32 = 3;
    const X: i32 = 1;
    const Y: i32 = 2;
    const Z: i32 = 3;

    let other = plays[0];
    let player_end = plays[1];
    let mut player = 0;
    let mut score: i32 = 0;
    
    if player_end == X {
        score = 0;
        if other == ROCK {
            player = SCISSORS;
        } else if other == PAPER {
            player = ROCK;
        } else if other == SCISSORS {
            player = PAPER;
        }    
    } else if player_end == Y {
        score = 3;
        player = other;
    } else if player_end == Z {
        score = 6;
        if other == ROCK {
            player = PAPER;
        } else if other == PAPER {
            player = SCISSORS;
        } else if other == SCISSORS {
            player = ROCK;
        }    
    }
    score += player;
    return score;
}
