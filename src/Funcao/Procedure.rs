#![allow(unused)]
// Funções simples
fn main() {
    fn double(mut x: f64) -> f64 {
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
    println!("{:.2}", x3)
}

// Funções simples usando High Level

fn main() {
    fn e1_area_retangulo(a: i64, b: i64) -> i64 {
        return a * b;
    }
    let x: i64 = e1_area_retangulo(10, 20);
    println!("{}", x);

    fn e2_media(a: f64, b: f64, c: f64) -> f64 {
        let media: f64 = (a + b + c) / 3.;
        return media;
    }
    let media: f64 = e2_media(5.5, 8.3, 10.0);
    // println!("{}", media.round());
    println!("{:.2}", media);

    fn e3_hipotenusa(ca: i64, co: i64) -> f64 {
        return ((ca.pow(2) + co.pow(2)) as f64).sqrt();
    }
    let x: f64 = e3_hipotenusa(3, 4);
    println!("{}", x);

    // fn e4_bhaskara(a: i64, b: i64, c: i64) -> (f64, f64) {
    //     let delta: f64 = b.pow(2) - 4 * a * c;
    //     return (1 + 2, 5 - 5);
    // }
}
