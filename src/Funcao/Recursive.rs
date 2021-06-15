

pub fn e2_Fatorial_InterativoHelper(mut n:f64,mut fat:f64) -> f64{
    if n == 1_f64 {
        return fat;
    }
    else{
        return e2_Fatorial_InterativoHelper(n - 1_f64, fat * n);
    }
}

pub fn e2_Fatorial_Interativo(mut n:f64) -> f64{
    return e2_Fatorial_InterativoHelper(n,1_f64);
}