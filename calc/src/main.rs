use std::io;

fn readInt() -> i32 {
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read input");
        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => return i,
            Err(..) => {
                println!("Invalid input, try again: ");
                continue;
            }
        };
    }
}

struct Calc {
    a: i32,
    b: i32,
}

impl Calc {
    pub fn get_inputs(&mut self) {
        loop {
            let mut num = String::new();
            io::stdin()
                .read_line(&mut num)
                .expect("Error while reading std input");
            let trimmed = num.trim();
            match trimmed.parse::<i32>() {
                Ok(i) => {
                    self.a = i;
                    break;
                }
                Err(..) => {
                    println!("Invalid Input, try again: ");
                    continue;
                }
            }
        }
        loop {
            let mut num = String::new();
            io::stdin()
                .read_line(&mut num)
                .expect("Error while reading std input");
            let trimmed = num.trim();
            match trimmed.parse::<i32>() {
                Ok(i) => {
                    self.b = i;
                    break;
                }
                Err(..) => {
                    println!("Invalid Input, try again: ");
                    continue;
                }
            }
        }
    }

    pub fn addition(&mut self) -> i32 {
        return self.a + self.b;
    }

    pub fn division(&mut self) -> f32 {
        if self.b == 0 && self.a != 0 {
            panic!("null exception occurrent");
        }
        return (self.a / self.b) as f32;
    }
}

fn main() {
    let mut c = Calc { a: 0, b: 0 };
    c.get_inputs();
    let mut add_value = c.addition();
    println!("add value: {} + {} = {}", c.a, c.b, add_value);
    let mut div_value = c.division();
    println!("div value: {} / {} = {}", c.a, c.b, div_value);
}
