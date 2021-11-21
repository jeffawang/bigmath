use num::bigint::{ToBigInt, BigInt};
use core::fmt;

struct Modulo<'a> {
    number: BigInt,
    modulo: &'a BigInt,
}

impl fmt::Display for Modulo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.number, self.modulo)
    }
}

impl<'a> Modulo<'a> {
    fn new(n : &i32, m : &'a BigInt) -> Modulo<'a> {
        Modulo{
            number: n.to_bigint().unwrap(),
            modulo: m,
        }
    }
}

impl<'a> std::ops::Add<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;
    fn add(self, rhs: &Modulo) -> Modulo<'a> {
        let out = (&self.number + &rhs.number) % self.modulo;
        Modulo{
            number: out,
            modulo: self.modulo,
        }
    }
}

impl<'a> std::ops::Add<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;
    fn add(self, rhs: Modulo) -> Modulo<'a> {
        let out = (&self.number + &rhs.number) % self.modulo;
        Modulo{
            number: out,
            modulo: self.modulo,
        }
    }
}

impl<'a> std::ops::Mul<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;
    fn mul(self, rhs: &Modulo) -> Modulo<'a> {
        let out = (&self.number * &rhs.number) % self.modulo;
        Modulo{
            number: out,
            modulo: self.modulo,
        }
    }
}

impl<'a> std::ops::Neg for &Modulo<'a> {
    type Output = Modulo<'a>;
    fn neg(self) -> Modulo<'a> {
        let out = (-&self.number) % self.modulo;
        Modulo{
            number: out,
            modulo: self.modulo,
        }
    }
}

impl<'a> std::ops::Sub<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;
    fn sub(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self + (-rhs)
    }
}

fn main() {
    let m = 65537.to_bigint().unwrap();
    let x = Modulo::new(&123, &m);
    let y = Modulo::new(&123, &m);
    println!("{}", &x + y);
}
