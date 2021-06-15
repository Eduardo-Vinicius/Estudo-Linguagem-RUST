/**
fn e1_ParOuImpar(j1:i32,e1:&str,j2:i32) -> &str{
    let soma = j1 + j2;
    let resultado = match soma % 2 == 0{
        true = "Par",
        false = "Ímpar",
    }
    let mut situacao = " ";

    if resultado == "Par"{
        if (e1 == "Par"){
            situacao = "Deu Par! Jogador 1 Venceu";
        }
        else{
            situacao = "Deu Par! Jogador 2 Venceu";
        }
    }
    else{
        if e1 == "Ímpar"{
            situacao = "Deu Ímpar! Jogador 1 Venceu";
        }
        else{
            situacao = "Deu Ímpar! Jogador 2 Venceu";
        }
    }
}
**/

pub fn e2_RecomendacaoMusica(pais:&str, estilo:&str) -> String{
    let mut musica = "";
    match &pais {
        &"Brasil" => match &estilo{
            &"MBP" => musica = "Azul da Cor do mar",
            &"Trap" => musica = "Tomodachi",
        }
        &"EUA" => musica = "Any music",
    }
    return musica.to_string();
}