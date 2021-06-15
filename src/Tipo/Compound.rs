struct Pessoa{
    nome:String;
}

struct Familia{
    pai:Pessoa;
    mae:Pessoa;
}

fn e1_Declaracao(){
    //
    let pessoa = Pessoa{
        nome:"Bruno".to_string(),
    };
    let x1 = pessoa.nome;

    //
    let pessoa2 = Pessoa{
        nome:"Bruno".to_string();
    };

    //
    let familia = Familia{
        pai:Pessoa{ Nome:"Junior".to_string() },
        mae:Pessoa{ Nome:"Maria".to_string() } 
    };
    let x2 = familia.pai.nome;
    let x3 = familia.mae.nome;

    //
    Pessoa pessoa3 = pessoa;
    pessoa3.nome = "Ruan";
}

fn e2_ValoresNull(){
    let pessoa = Pessoa;

}

fn e3_TiposAnonimos(){
    let pessoa = { nome = "Bruno" };

    let familia = {
        pai = { nome = "Junior" },
        mae = { nome = "Maria" }
    };

    let x1 = familia.pai.nome;
    let x2 = familia.mae.nome;
}

fn e4_Alteracao(){
    let mut p1 = Pessoa{
        nome = "Bruno"
    };
    p1.nome = "Bruno V.";

    let x1 = "Viana";
    p1.nome = x1;
    p1.nome = format!{"Bruno {}",x1};

    let x2 = p1.nome;
}

fn e5_Comparacao(){
    let p1 = Pessoa { nome = "Bruno" };
    let p2 = Pessoa { nome = "Bruno" };

    let x1:bool = p1 == p2;

    //
    let p3 = Pessoa { nome = "Bruno" };
    let mut p4 = p3;
    p4.nome = "Ruan";

    let x12:bool = p3 == p4;
}