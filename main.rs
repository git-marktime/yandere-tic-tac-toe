use std::io;
use terminal::{Clear, Action};

fn printboard(positions_array: Vec<char>) {
    println!("  a b c");
    println!("1 {} {} {}",positions_array[0],positions_array[1],positions_array[2]);
    println!("2 {} {} {}",positions_array[3],positions_array[4],positions_array[5]);
    println!("3 {} {} {}",positions_array[6],positions_array[7],positions_array[8]);
}

fn getinput() -> String {
    loop {
        let mut userinputbuffer = String::new();
        io::stdin().read_line(&mut userinputbuffer).expect("broke while reading line");
        let userinputbuffer = userinputbuffer.trim();

        if userinputbuffer.len() == 2 && userinputbuffer.split_at(1).0.is_ascii() && userinputbuffer.split_at(1).1.parse::<i32>().is_ok() {
            return userinputbuffer.to_string();
        } else {
            println!("try again");
        }
    }
}

fn handleinput(mut positions_array: Vec<char>, userinput: String, turnchar: char) -> Vec<char> {
    let userinput = userinput.as_str();

    if userinput == "a1" && positions_array[0] == '.'{
        positions_array[0] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "b1" && positions_array[1] == '.' {
        positions_array[1] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "c1" && positions_array[2] == '.' {
        positions_array[2] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "a2" && positions_array[3] == '.' {
        positions_array[3] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "b2" && positions_array[4] == '.' {
        positions_array[4] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "c2" && positions_array[5] == '.' {
        positions_array[5] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "a3" && positions_array[6] == '.' {
        positions_array[6] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "b3" && positions_array[7] == '.' {
        positions_array[7] = turnchar;
        positions_array[9] = 'W';
    } else if userinput == "c3" && positions_array[8] == '.' {
        positions_array[8] = turnchar;
        positions_array[9] = 'W';
    } else {
        println!("error");
        positions_array[9] = 'E';
    }
        
    positions_array
}

fn checkwin(positions_array: Vec<char>) -> char {
    if positions_array.contains(&'.') {
        if positions_array[0] == 'x' && positions_array[1] == 'x' && positions_array[2] == 'x' ||
        positions_array[0] == 'x' && positions_array[3] == 'x' && positions_array[6] == 'x' ||
        positions_array[2] == 'x' && positions_array[5] == 'x' && positions_array[8] == 'x' ||
        positions_array[6] == 'x' && positions_array[7] == 'x' && positions_array[8] == 'x' ||
        positions_array[0] == 'x' && positions_array[4] == 'x' && positions_array[8] == 'x' ||
        positions_array[2] == 'x' && positions_array[4] == 'x' && positions_array[6] == 'x' ||
        positions_array[1] == 'x' && positions_array[4] == 'x' && positions_array[7] == 'x' ||
        positions_array[3] == 'x' && positions_array[4] == 'x' && positions_array[5] == 'x'
        {
            'x'
        } else if positions_array[0] == 'o' && positions_array[1] == 'o' && positions_array[2] == 'o' ||
        positions_array[0] == 'o' && positions_array[3] == 'o' && positions_array[6] == 'o' ||
        positions_array[2] == 'o' && positions_array[5] == 'o' && positions_array[8] == 'o' ||
        positions_array[6] == 'o' && positions_array[7] == 'o' && positions_array[8] == 'o' ||
        positions_array[0] == 'o' && positions_array[4] == 'o' && positions_array[8] == 'o' ||
        positions_array[2] == 'o' && positions_array[4] == 'o' && positions_array[6] == 'o' ||
        positions_array[1] == 'o' && positions_array[4] == 'o' && positions_array[7] == 'o' ||
        positions_array[3] == 'o' && positions_array[4] == 'o' && positions_array[5] == 'o'
        {
            'o'
        } else {
            'n'
        }
    } else {
        'c'
    }
}

fn clearwindow() {
    let _ = terminal::stdout().act(Action::ClearTerminal(Clear::All));
}

fn main() {
    let mut positions_array: Vec<char> = vec![
        '.','.','.',
        '.','.','.',
        '.','.','.', 'W'
    ];

    let mut turn = 'x';

    loop {
        if positions_array[9] != 'E'{
            clearwindow();
            printboard(positions_array.clone());
        }

        println!("put in a position ({}): ", turn);
        let userinput = getinput();
        positions_array = handleinput(positions_array.clone(), userinput.clone(), turn);

        let winchar = checkwin(positions_array.clone());
        if winchar == 'c' {
            clearwindow();
            printboard(positions_array);
            println!("tie");
            positions_array = vec![
                '.','.','.',
                '.','.','.',
                '.','.','.', 'W'
            ];
            println!("input any key to continue: ");
            let mut pee = String::new();
            io::stdin().read_line(&mut pee).expect("broek");
        } else if winchar == 'o' {
            clearwindow();
            printboard(positions_array);
            println!("o wins");
            positions_array = vec![
                '.','.','.',
                '.','.','.',
                '.','.','.', 'W'
            ];
            println!("input any key to continue: ");
            let mut pee = String::new();
            io::stdin().read_line(&mut pee).expect("broek");
        } else if winchar == 'x' {
            clearwindow();
            printboard(positions_array);
            println!("x wins");
            positions_array = vec![
                '.','.','.',
                '.','.','.',
                '.','.','.', 'W'
            ];
            println!("input any key to continue: ");
            let mut pee = String::new();
            io::stdin().read_line(&mut pee).expect("broek");
        }


        if turn == 'x' && positions_array[9] != 'E' {
            turn = 'o';
        } else if turn == 'o' && positions_array[9] != 'E' {
            turn = 'x';
        } else {
            
        }
    }
}