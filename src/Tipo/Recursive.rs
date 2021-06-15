

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

fn e2_Lista(){
    //
    let mut lista: LinkedList<u32> = LinkedList::new();
    lista.push_back(10);
    lista.push_back(20);
    lista.push_back(30);

    //
    let a = &lista[0];
    let b = &lista[1];
    let c = &lista[2];

    //
    lista[0] = 100;
    
    //
    let x = lista.Count;
    let x = lista.len();

    //
    lista.pop_back(2);
    lista.pop_front(10);
}

fn e3_Fila(){
    let fila: VecDeque(u32) = VecDeque::new();
    fila.push_back(10);
    fila.push_back(20);
    fila.push_back(30);

    
}