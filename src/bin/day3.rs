use advent_of_code_2024::aoc_main;

enum Token {
    Mul(usize, usize),
    Cond(bool),
}

enum ParserFSM {
    Nothing,
    D,
    O,
    N,
    Apostrophe,
    T,
    CondLeftPar(bool),
    M,
    U,
    L,
    MulLeftPar,
    LeftDigit0(usize),
    LeftDigit1(usize),
    LeftDigit2(usize),
    Comma(usize),
    RightDigit0(usize, usize),
    RightDigit1(usize, usize),
    RightDigit2(usize, usize),
}

impl ParserFSM {
    fn consume(&mut self, c: char) -> Option<Token> {
        match (&self, c) {
            (_, 'd') => *self = Self::D,
            (Self::D, 'o') => *self = Self::O,
            (Self::O, 'n') => *self = Self::N,
            (Self::N, '\'') => *self = Self::Apostrophe,
            (Self::Apostrophe, 't') => *self = Self::T,
            (Self::O, '(') => *self = Self::CondLeftPar(true),
            (Self::T, '(') => *self = Self::CondLeftPar(false),
            (Self::CondLeftPar(cond), ')') => {
                let token = Token::Cond(*cond);
                *self = Self::Nothing;
                return Some(token);
            }
            (_, 'm') => *self = Self::M,
            (Self::M, 'u') => *self = Self::U,
            (Self::U, 'l') => *self = Self::L,
            (Self::L, '(') => *self = Self::MulLeftPar,
            (Self::MulLeftPar, x) if x.is_ascii_digit() => {
                *self = Self::LeftDigit0(x.to_digit(10).unwrap() as usize);
            }
            (Self::LeftDigit0(x_part), x) if x.is_ascii_digit() => {
                *self = Self::LeftDigit1(*x_part * 10 + x.to_digit(10).unwrap() as usize);
            }
            (Self::LeftDigit1(x_part), x) if x.is_ascii_digit() => {
                *self = Self::LeftDigit2(*x_part * 10 + x.to_digit(10).unwrap() as usize);
            }
            (Self::LeftDigit0(x), ',')
            | (Self::LeftDigit1(x), ',')
            | (Self::LeftDigit2(x), ',') => *self = Self::Comma(*x),
            (Self::Comma(x), y) if y.is_ascii_digit() => {
                *self = Self::RightDigit0(*x, y.to_digit(10).unwrap() as usize);
            }
            (Self::RightDigit0(x, y_part), y) if y.is_ascii_digit() => {
                *self = Self::RightDigit1(*x, *y_part * 10 + y.to_digit(10).unwrap() as usize);
            }
            (Self::RightDigit1(x, y_part), y) if y.is_ascii_digit() => {
                *self = Self::RightDigit2(*x, *y_part * 10 + y.to_digit(10).unwrap() as usize);
            }
            (Self::RightDigit0(x, y), ')')
            | (Self::RightDigit1(x, y), ')')
            | (Self::RightDigit2(x, y), ')') => {
                let token = Token::Mul(*x, *y);
                *self = Self::Nothing;
                return Some(token);
            }
            _ => *self = Self::Nothing,
        }
        None
    }
}

fn common(input: String) -> Vec<Token> {
    let mut fsm = ParserFSM::Nothing;
    let mut tokens = Vec::new();

    for c in input.chars() {
        if let Some(token) = fsm.consume(c) {
            tokens.push(token);
        }
    }

    tokens
}

fn part1(tokens: &Vec<Token>) -> usize {
    let mut result = 0;

    for token in tokens {
        if let Token::Mul(x, y) = token {
            result += x * y;
        }
    }

    result
}

fn part2(tokens: &Vec<Token>) -> usize {
    let mut result = 0;
    let mut cond = true;

    for token in tokens {
        match token {
            Token::Mul(x, y) if cond => result += x * y,
            Token::Cond(new_cond) => cond = *new_cond,
            _ => {}
        }
    }

    result
}

aoc_main!(3, common, part1, part2);
