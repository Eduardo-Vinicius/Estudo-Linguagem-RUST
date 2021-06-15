

pub fn e1_RecomendacaoMusica(pais:&str, estilo:&str) -> String{
    let mut musica = "";
    match pais {
        "Brasil" => match estilo{
            "MBP" => musica = "Azul da Cor do mar",
            "Trap" => musica = "Tomodachi",
            _ => musica = "qualquer musica",
        }
        "EUA" => musica = "Any music",
        _ => musica = "qualquer musica",
    }
    return musica.to_string();
}