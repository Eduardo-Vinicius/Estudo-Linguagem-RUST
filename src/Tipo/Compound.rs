

struct Pessoa{
    nome:String,
}

struct Familia{
    pai:Pessoa,
    mae:Pessoa,
}

pub fn e1_Declaracao(){
    //
    let mut pessoa = Pessoa{
        nome:"Bruno".to_string(),
    };
    let x1:String = pessoa.nome;

    //
    let pessoa2 = Pessoa{
        nome:"Bruno".to_string(),
    };

    //
    let familia = Familia{
        pai:Pessoa{ nome:"Junior".to_string() },
        mae:Pessoa{ nome:"Maria".to_string() } 
    };
    let x2 = familia.pai.nome;
    let x3 = familia.mae.nome;

    //
    let mut pessoa3 = pessoa;
    pessoa3.nome = "Ruan".to_string();
}




pub fn e2_Alteracao(){
    let mut p1 = Pessoa{
        nome:"Bruno".to_string(),
    };
    p1.nome = "Bruno V.".to_string();

    let x1:String = "Viana".to_string();
    p1.nome = x1;
    p1.nome = format!{"Bruno {}",x1};

    let x2 = p1.nome;
}

pub fn e3_Comparacao(){
    let p1 = Pessoa { nome:"Bruno".to_string() };
    let p2 = Pessoa { nome:"Bruno".to_string() };

    let x1:bool = p1.nome == p2.nome;

    //
    let mut p3 = Pessoa { nome:"Bruno".to_string() };
    let mut p4 = p3;
    p4.nome = "Ruan".to_string();

    let x12:bool = p3.nome == p4.nome;
}