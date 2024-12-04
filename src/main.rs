use std::io::BufReader;
use std::fs::File;
use std::io::Read;

pub fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut acccumulated_state = AccumulatedState{ multiplicand_string: String::new(), multiplier_string: String::new(), sum_of_multiplier_instructions: 0, do_enabled: true};
    let mut state: Box<dyn State> = Box::new(OuterState);

    for byte in reader.bytes() {
        match byte {
            // byte is exactly one byte
            Ok(byte) => state = state.consume(& mut acccumulated_state, byte),
            Err(err) => println!("Error reading a byte: {err}"),
        }
    }

    println!("sum_of_multiplier_instructions {}", acccumulated_state.sum_of_multiplier_instructions);

    Ok(())
}

// With thanks to https://refactoring.guru/design-patterns/state/rust/example


struct AccumulatedState {
    multiplicand_string : String,
    multiplier_string : String,
    sum_of_multiplier_instructions : i64,
    do_enabled : bool
}


pub struct OuterState;
pub struct ReadmState;
pub struct ReadmuState;
pub struct ReadmulState;
pub struct ReadmulOpenParenthesisState;
pub struct ReadDigitMultiplicandState;
pub struct ReadCommaState;
pub struct ReadDigitMultiplierState;
pub struct ReaddState;
pub struct ReaddoState;
pub struct ReaddoOpenParenthesisState;
pub struct ReaddoCloseParenthesisState;
pub struct ReaddonState;
pub struct ReaddonApostropheState;
pub struct ReaddonApostrophetState;
pub struct ReaddonApostrophetOpenParenthesisState;
pub struct ReaddonApostrophetCloseParenthesisState;



trait State {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State>;
}

impl State for OuterState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("OuterState {byte}");

        if byte == b'm' {
            return Box::new(ReadmState);
        } else if byte == b'd' {
            return Box::new(ReaddState);
        }
        self
    }
}

impl State for ReadmState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b'u'   {
            return Box::new(ReadmuState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadmuState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReaduState {byte}");

        if byte == b'l' {
            return Box::new(ReadmulState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadmulState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadlState {byte}");

        if byte == b'(' {
            return Box::new(ReadmulOpenParenthesisState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadmulOpenParenthesisState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadOpenParenthesisState {byte}");

        if byte.is_ascii_digit() {
            accumulated_state.multiplicand_string.clear();
            accumulated_state.multiplicand_string.push(char::from_u32(byte as u32).unwrap());
            return Box::new(ReadDigitMultiplicandState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadDigitMultiplicandState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadDigitMultiplicandState {byte}");

        if byte.is_ascii_digit() {
            accumulated_state.multiplicand_string.push(char::from_u32(byte as u32).unwrap());
            return self;
        } else if byte == b',' {
            return Box::new(ReadCommaState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadCommaState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadCommaState {byte}");

        if byte.is_ascii_digit() {
            accumulated_state.multiplier_string.clear();
            accumulated_state.multiplier_string.push(char::from_u32(byte as u32).unwrap());
            return Box::new(ReadDigitMultiplierState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadDigitMultiplierState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadDigitMultiplierState {byte}");

        if byte.is_ascii_digit() {
            accumulated_state.multiplier_string.push(char::from_u32(byte as u32).unwrap());
            return self;
        } else if byte == b')' {
            let multiplicand: i64 = accumulated_state.multiplicand_string.parse().unwrap();
            let multiplier: i64 = accumulated_state.multiplier_string.parse().unwrap();
            let product = multiplicand * multiplier;
            if accumulated_state.do_enabled  {
                accumulated_state.sum_of_multiplier_instructions += product;
            }
            println!("ReadDigitMultiplierState read ')' multiplicand {multiplicand} multiplier {multiplier} product {product} sum_of_multiplier_instructions {}", accumulated_state.sum_of_multiplier_instructions);
            return Box::new(OuterState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b'o'   {
            return Box::new(ReaddoState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddoState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b'(' {
            return Box::new(ReaddoOpenParenthesisState);
        } else if byte == b'n'   {
            return Box::new(ReaddonState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddoOpenParenthesisState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b')' {
            accumulated_state.do_enabled = true;
            return Box::new(OuterState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddonState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b'\'' {
            return Box::new(ReaddonApostropheState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddonApostropheState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b't' {
            return Box::new(ReaddonApostrophetState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddonApostrophetState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b'(' {
            accumulated_state.do_enabled = true;
            return Box::new(ReaddonApostrophetOpenParenthesisState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaddonApostrophetOpenParenthesisState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b')' {
            accumulated_state.do_enabled = false;
            return Box::new(OuterState);
        } else {
            return Box::new(OuterState);
        }
    }
}
