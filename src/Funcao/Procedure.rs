#![allow(unused)]
// Funções simples
fn main() {
    fn double(mut x: f64 ) -> f64 {
        x *= 2.;
        return (x);
    }
    fn somar(mut x: f64, mut y: f64) -> f64 {
        let soma: f64 = x + y;
        return (soma);
    }
    fn subtrair(mut x: f64, mut y: f64) -> f64 {
        let sub: f64 = x - y;
        return (sub);
    }
    let x1: f64 = double(20.0);
    let x2: f64 = somar(30.0, 40.0);
    let x3: f64 = subtrair(40.0);
    println!("{}", x3)
}

// Funções simples usando High Level

fn main() {

    fn hipotenusa(ca: i64, co: i64) -> f64{
        return ((ca.pow(2) + co.pow(2)) as f64).sqrt();
    }
    
    let x: f64 = hipotenusa(3, 4);
    println!("{}", x);
}
