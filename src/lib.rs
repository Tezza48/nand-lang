use std::collections::VecDeque;

mod tests {
    #[test]
    fn test_test() {}
}

type Word = i32;

#[derive(Debug)]
enum Token {
    Constant(Word),
    Pop,
    Nand,
    Store,
    Load,
    Swap,
    Jump,
}

pub fn start(source: String) -> Result<(), String> {
    let mut tokens = Vec::new();

    let split = source.split_whitespace();
    for token in split {
        println!("Token: {}", token);

        tokens.push(match token {
            "n" => Token::Nand,
            "*" => Token::Pop,
            ">" => Token::Store,
            "<" => Token::Load,
            "%" => Token::Swap,
            "!" => Token::Jump,
            _ => {
                if token.starts_with("0x") {
                    let rest = token.trim_start_matches("0x");
                    println!("rest: {}", rest);
                    let hex = Word::from_str_radix(rest, 16).unwrap();
                    Token::Constant(hex)
                } else {
                    Token::Constant(token.parse::<Word>().unwrap())
                }
            }
        });
    }

    println!("tokens: {:?}", &tokens);

    let mut store: Word = 0;
    let mut swap: Word = 0;
    let mut prog_counter = 0;

    let mut memory: VecDeque<Word> = VecDeque::new();

    while prog_counter < tokens.len() {
        let token = &tokens[prog_counter];

        let mut should_increment_prog_counter = true;

        match token {
            Token::Constant(value) => memory.push_front(*value),
            Token::Pop => drop(memory.pop_front()),
            Token::Nand => {
                if let (Some(a), Some(b)) = (memory.pop_front(), memory.pop_front()) {
                    let result = !(a & b);
                    memory.push_front(a);
                    memory.push_front(b);
                    memory.push_front(result);
                }
            }
            Token::Store => {
                let ptr = memory.pop_front().unwrap();
                let value = memory.iter().nth(ptr as usize).unwrap();
                store = *value;
            }
            Token::Swap => std::mem::swap(&mut store, &mut swap),
            Token::Load => {
                memory.push_front(store);
            }
            Token::Jump => {
                let value = memory.pop_front().unwrap();
                let new_counter = memory.pop_front().unwrap();
                if value == 0 {
                    prog_counter = new_counter as usize;
                    should_increment_prog_counter = false;
                }
            }
        };

        println!(
            "State\n\tProgram Counter: {}, Token: {:?}, Store/Swap: {}/{}\n\tMemory: {:?}\n",
            &prog_counter, token, &store, &swap, &memory
        );

        if should_increment_prog_counter {
            prog_counter += 1;
        }
    }

    Ok(())
}
