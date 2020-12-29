use std::fs;

pub fn day18() {
    let contents = fs::read_to_string("Input/Day18.txt").expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
        let tokens = line.replace(' ', "");
        let tokens = tokens.chars().collect();

        part1 += calculate(&tokens);
        part2 += calculate2(&tokens);
    }

     println!("Part 1: {}", part1);
     println!("Part 2: {}", part2);
}

fn calculate(tokens: &Vec<char>) -> u128 {
    let (leftside, new_index) = parse(tokens, 0);
    let mut result = leftside;
    let mut index = new_index;
    while index < tokens.len() {
        let operator = tokens[index];
        if !is_operator(operator) {
            panic!("Expected operator but got {} at index {} of {:?}", operator, index, tokens);
        }
        let (rightside, new_index) = parse(tokens, index + 1);
        index = new_index;
        if operator == '+' {
            result += rightside;
        } else {
            result *= rightside;
        }
    }
    result
}

fn is_operator(c: char) -> bool { c == '+' || c == '*' }

fn parse(tokens: &Vec<char>, index: usize) -> (u128, usize){
    let token = tokens[index];
    if token.is_numeric() {
        return (token.to_digit(10).unwrap() as u128, index + 1);
    }
    if token != '(' {
        panic!("Expected ( but got {} at index {} of {:?}", token, index, tokens);
    }

    let (leftside, new_index) = parse(tokens, index + 1);
    let mut result = leftside;
    let mut index = new_index;

    while index < tokens.len() {
        let operator = tokens[index];
        if operator == ')' {
            return (result, index + 1);
        }
        if !is_operator(operator) {
            panic!("parse() Expected operator but got {} at index {} of {:?}", operator, index, tokens);
        }
        let (rightside, new_index) = parse(tokens, index + 1);
        index = new_index;
        if operator == '+' {
            result += rightside;
        } else {
            result *= rightside;
        }
    }
    panic!("parse() reached the end at index {} of {:?}", index, tokens);
}

const PLUS: u128 = u128::max_value();
const TIMES: u128 = u128::max_value() -1;
const OPEN: u128 = u128::max_value() - 2;
const CLOSE: u128 = u128::max_value() - 3;

fn is_number(n: u128) -> bool { n < CLOSE }

fn calculate2(tokens: &Vec<char>) -> u128 {
    let mut tokens:Vec<u128> = tokens.into_iter().
        map(|c| match c {
            '+' => PLUS,
            '*' => TIMES,
            '(' => OPEN,
            ')' => CLOSE,
            _ => c.to_digit(10).unwrap() as u128
        }).collect();

    while tokens.len() > 1 {
        calculate_a_parenthesis(&mut tokens);
    }
    tokens[0]
}

fn calculate_a_parenthesis(tokens: &mut Vec<u128>) {
    let mut open: usize = 0;
    let mut close: usize = tokens.len();
    for i in 0..tokens.len() {
        if tokens[i] == OPEN { open = i; }
        if tokens[i] == CLOSE {
            close = i;
            break;
        }
    }
    calculate2_flat(tokens, open, close);
}

fn calculate2_flat(tokens: &mut Vec<u128>, open: usize, close: usize) {
    let start = open;
    let mut end = close;
    while start + 1 < end {
        let mut did_calculate = false;
        for i in start..end - 1 {
            if tokens[i] == OPEN && is_number(tokens[i + 1]) && tokens[i + 2] == CLOSE {
                tokens.remove(i + 2);
                tokens.remove(i);
                end -= 2;
                did_calculate = true;
                break;
            }
        }
        if did_calculate { continue; }

        for i in start..end - 1 {
            if is_number(tokens[i]) && tokens[i + 1] == PLUS && is_number(tokens[i + 2]) {
                tokens[i] += tokens[i + 2];
                tokens.remove(i + 2);
                tokens.remove(i + 1);
                end -= 2;
                did_calculate = true;
                break;
            }
        }
        if did_calculate { continue; }

        for i in start..end - 1 {
            if is_number(tokens[i]) && tokens[i + 1] == TIMES && is_number(tokens[i + 2]) {
                tokens[i] *= tokens[i + 2];
                tokens.remove(i + 2);
                tokens.remove(i + 1);
                end -= 2;
                did_calculate = true;
                break;
            }
        }
        if !did_calculate { panic!("Tokens {:?}, start {}, end {}", pretty_print(tokens), start, end); }
    }
}

fn pretty_print(tokens: &[u128]) -> String {
    let mut s = String::from("[");
    for token in tokens {
        s += match *token {
            PLUS => "+",
            TIMES => "*",
            OPEN => "(",
            CLOSE => ")",
            _ => ""
        };
        if is_number(*token) {
            s += &*token.to_string();
        }
    }
    s + "]"
}
// Part 1: 9535936849815
//         9535936849815
// Part 2: 472171581333710
//         472171581333710
