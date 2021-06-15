#![allow(unused)]
// Funções simples
pub fn double(mut x: f64) -> f64 {
    x *= 2.;
    return (x);
}
pub fn somar(mut x: f64, mut y: f64) -> f64 {
    let soma: f64 = x + y;
    return (soma);
}
pub fn subtrair(mut x: f64, mut y: f64) -> f64 {
    let sub: f64 = x - y;
    return (sub);
}



// Funções simples usando High Level

pub fn e1_area_retangulo(a: i64, b: i64) -> i64 {
    return a * b;
}


pub fn e2_media(a: f64, b: f64, c: f64) -> f64 {
    let media: f64 = (a + b + c) / 3.;
    return media;
}


pub fn e3_hipotenusa(ca: i64, co: i64) -> f64 {
    return ((ca.pow(2) + co.pow(2)) as f64).sqrt();
}

// fn e4_bhaskara(a: i64, b: i64, c: i64) -> (f64, f64) {
//     let delta: f64 = b.pow(2) - 4 * a * c;
//     return (1 + 2, 5 - 5);
// }

