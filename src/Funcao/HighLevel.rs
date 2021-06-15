
pub fn e1_FuncoesParaNumeros(){
    let x = 10.1234_f32;
    let y = 9_f32;

    let x1 = x.floor();
    let x2 = x.ceil();
    let x3 = x.trunc();
    let x4 = x.round();
    let x5 = x.abs();
    let x6 = x.sqrt();
    let x7 = x.powf(2_f32);
}

pub fn e2_FuncoesParaTexto(){
    let msg = "Olá! Tudo bem?";

    let x1 = msg.to_uppercase();
    let x2 = msg.to_lowercase();
    let x3 = msg.trim();
    let x4 = msg.replace("Olá", "Oiee");
    let x5 = msg.contains("bem?");
    let x9 = msg.len();

    let x10 = ["Olá!", "Tudo", "bem?"];
    let x11 = format!("{:?}",x10);

}

use std::collections::LinkedList;
pub fn e3_FuncoesParaTiposRecursivos(){
    let mut lista:LinkedList<i32> = LinkedList::new();
    lista.push_back(1);
    lista.push_back(2);
    lista.push_back(3);
    lista.push_back(4);
    lista.push_back(5);
    lista.push_back(6);
    lista.push_back(7);
    lista.push_back(8);
    lista.push_back(9);
    lista.push_back(10);


    let x1 = lista.len();
    let x2 = lista.contains(&5);
    
}

pub fn e4_FuncoesParaConversao(){
    let x = "10.5";
    //
    let x1 = x.parse::<u32>().unwrap();
    let x2 = x.parse::<bool>().unwrap();
    let x3 = x.parse::<f32>().unwrap();
    let x4 = x.parse::<char>().unwrap();
    let x5 = x.to_string();
}