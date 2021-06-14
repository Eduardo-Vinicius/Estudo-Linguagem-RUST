fn e1_IF_Aprovado(m:f32) -> str {
    let mut situacao = "Reprovado";
    if m >= 5_f32 {
        situacao = "Aprovado";
    }
    else if m >= 3.5 {
        situacao = "Recuperação";
    }
    else{
        situacao = "Reprovado";
    }
    return situacao;
}

fn e2_IF_Aprovado(m:f32,f:i32) -> str{
    let mut situacao = "";
    if m < 5 && f >= 25{
        situacao = "Reprovado por nota e falta";
    }
    else if f >= 25 {
        situacao = "Reprovado por falta";
    }
    else if m < 5 {
        situacao = "Reprovado por nota";
    }
    else{
        situacao = "Aprovado"
    }
    return situacao;
}

fn e3_Switch_Avaliacao(n:str) -> str{
    let mut situacao = "";

    match n{
        "A" => situacao = "Otimo",
        "B" => situacao = "Bom",
        _ => situacao = "Ruim",
    }
    return situacao;
}

fn e4_Switch_Verdadeiro(){
    let booleano = false;
    let binario = match booleano{
        false => 0,
        true => 1,
    }
}