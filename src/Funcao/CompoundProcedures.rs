
pub fn mainProcedure() {
    #[derive(Debug)]
    struct Retangulo {
        altura: i64,
        base: i64,
    }

    let retangulo1 = Retangulo {
        altura: 10,
        base: 15,
    };

    let retangulo2 = Retangulo {
        altura: 10,
        base: 15,
    };

    fn e1_area_retangulo(a: i64, b: i64) -> i64 {
        return a * b;
    }

    fn e2_retangulos_iguais(r1: Retangulo, r2: Retangulo) -> bool {
        return e1_area_retangulo(r1.altura, r1.base) == e1_area_retangulo(r2.altura, r2.base);
    }

    let x: bool = e2_retangulos_iguais(retangulo1, retangulo2);
    println!("{}", x)
}
