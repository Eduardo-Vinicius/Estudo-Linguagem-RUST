use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::LinkedList;

fn e1_Vetor(){
    //
    let mut vetor:Vec<i32> = Vec::new();
    vetor.push(10);
    vetor.push(20);
    vetor.push(30);

    //
    let a = &vetor[0];
    let b = &vetor[1];
    let c = &vetor[2];

    //
    let x = vetor.len();
}

fn e3_Fila(){
    let fila: VecDeque(u64) = VecDeque::new();
    fila.push_front(10);
    fila.push_front(20);
    fila.push_front(30);

    // Remove
    let a = fila.pop_front();​
    let b = fila.pop_front();​
    let c = fila.pop_front();

    let cumprimento = fila.len();
    
}

fn e4_Pilha() {
    let pilha: VecDeque(u64) = VecDeque::new();
    pilha.push_back(10);
    pilha.push_back(20);
    pilha.push_back(30);

    // Remove
    let a = pop_back();​
    let b = pop_back();
    let c = pop_back(); 

    let cumprimento = pilha.len();
}

fn e5_Dicionarios(){
    
    let mut dicionario: HashMap<i64, &str> = HashMap::new();​

    // Inserindo
    dicionario.insert(1, "C#");​
    dicionario.insert(2, "Javascript");
    dicionario.insert(3, "Python");

    // Lendo cada um
    let a = dicionario.get(1);​
    let b = dicionario.get(2);​
    let c = dicionario.get(3);

    // Iterando
    let x = dic.iter();

    // Update
    dicionario.insert(1, "Java");

    dicionario.remove(&1);
}