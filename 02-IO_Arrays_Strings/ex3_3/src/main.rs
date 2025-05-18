pub enum MulErr {
    Overflow, 
    NegativeNumber,
}

pub fn mul(a: i32, b:i32) -> Result<u32, MulErr>{
    if (a<0 && b>=0) || (b<0 && a>=0) {
        Err(MulErr::NegativeNumber)
    }
    else if a.checked_mul(b)==None {
        Err(MulErr::Overflow)
    }
    else {
        Ok((a*b) as u32)
    }

}

fn main() {
    match mul(2000, 200000) {
        Ok(res) => println!("Result: {}", res),
        Err(err) => match err {
            MulErr::Overflow => print!("Overflow detected"),
            MulErr::NegativeNumber => println!("Negative result")
        },
    }
}
