use std::io::BufReader;
use std::fs::File;
use std::io::Read;

pub fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut acccumulated_state = AccumulatedState{ multiplicand_string: String::new(), multiplier_string: String::new(), sum_of_multiplier_instructions: 0};
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
    sum_of_multiplier_instructions : i64
}


pub struct OuterState;
pub struct ReadmState;
pub struct ReaduState;
pub struct ReadlState;
pub struct ReadOpenParenthesisState;
pub struct ReadDigitMultiplicandState;
pub struct ReadCommaState;
pub struct ReadDigitMultiplierState;


trait State {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State>;
}

impl State for OuterState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("OuterState {byte}");

        if byte == b'm' {
            return Box::new(ReadmState);
        }
        self
    }
}

impl State for ReadmState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadmState {byte}");

        if byte == b'u'   {
            return Box::new(ReaduState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReaduState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReaduState {byte}");

        if byte == b'l' {
            return Box::new(ReadlState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadlState {
    fn consume(self: Box<Self>, accumulated_state: &mut AccumulatedState, byte: u8) -> Box<dyn State> {
        println!("ReadlState {byte}");

        if byte == b'(' {
            return Box::new(ReadOpenParenthesisState);
        } else {
            return Box::new(OuterState);
        }
    }
}

impl State for ReadOpenParenthesisState {
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
            accumulated_state.sum_of_multiplier_instructions += product;
            println!("ReadDigitMultiplierState read ')' multiplicand {multiplicand} multiplier {multiplier} product {product} sum_of_multiplier_instructions {}", accumulated_state.sum_of_multiplier_instructions);
            return Box::new(OuterState);
        } else {
            return Box::new(OuterState);
        }
    }
}
