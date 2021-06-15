use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::LinkedList;

pub fn e1_Vetor(){
    //
    let mut vetor:Vec<i32> = Vec::new();
    vetor.push(10);
    vetor.push(20);
    vetor.push(30);

    println!("{:?}",vetor);
    //
    let a = &vetor[0];
    let b = &vetor[1];
    let c = &vetor[2];
    println!("{},{},{}",a,b,c);
    //
    let x = vetor.len();
    println!("{}",x);
}

pub fn e2_Lista(){
    //
    let mut lista: LinkedList<u32> = LinkedList::new();
    lista.push_back(10);
    lista.push_back(20);
    lista.push_back(30);
    println!("{:?}",lista);
    //
    let a = &lista[0];
    let b = &lista[1];
    let c = &lista[2];
    
    //
    let x = lista.len();

    //
    lista.pop_back(2);
    lista.pop_front(10);
}

pub fn e3_Fila(){
    let fila: VecDeque(u64) = VecDeque::new();
    fila.push_front(10);
    fila.push_front(20);
    fila.push_front(30);
    println!("{:?}",fila);
    // Remove
    let a = fila.pop_front();​
    let b = fila.pop_front();
    let c = fila.pop_front();

    let cumprimento = fila.len();
    
}

pub fn e4_Pilha() {
    let pilha: VecDeque(u64) = VecDeque::new();
    pilha.push_back(10);
    pilha.push_back(20);
    pilha.push_back(30);
    println!("{:?}",pilha);
    // Remove
    let a = pop_back();​
    let b = pop_back();
    let c = pop_back(); 

    let cumprimento = pilha.len();
}

pub fn e5_Dicionarios(){
    
    let mut dicionario: HashMap<i64, &str> = HashMap::new();​

    // Inserindo
    dicionario.insert(1, "C#");​
    dicionario.insert(2, "Javascript");
    dicionario.insert(3, "Python");

    println!("{:?}",dicionario);
    // Lendo cada um
    let a = dicionario.get(&1);​
    let b = dicionario.get(&2);
    let c = dicionario.get(&3);

    // Iterando
    let x = dicionario.iter();
    println!("{:?}",x);

    // Update
    dicionario.insert(1, "Java");
    println!("{:?}",dicionario);

    //
    dicionario.remove(&1);
    println!("{:?}",dicionario);
}