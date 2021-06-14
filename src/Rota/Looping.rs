
fn e1_loopexpression(){
    loop { println!("I live."); } //loop infinito
    loop { println!("I live."); break;} //loop com uma volta
}

fn e2_WHILE_SomarNumeros(fim:i32) -> i32{
    let mut soma = 0;
    let i = 0;
    while i <= fim {
        soma += 1; 
    }
    return soma;
}

fn e3_FOR_FiltrarPares(mut num:Vec<i32>) -> LinkedList{
    let mut pares:LinkedList<i32> = LinkedList::new();
    for item in &num{
        if item % 2 == 0{
            pares.push_back(item);
        }
    }
    return pares;
}

fn e4_CONTINUE_GerarSeqeunciaPar(fim:i32) -> LinkedList{
    let mut seq:LinkedList<i32> = LinkedList::new();
    while i <= fim {
        if i % 2 != 0{
            continue;
        }
        seq.push_back(i);
    } 
    return seq; 
}

fn e5_WHILE_ProximaRaizInteira(mut num:i32) -> i32{
    let mut resto:f32 = 1.0;
    while resto != 0{
        num + 1;
        resto = num.sqrt() % 1;
    }
    return num;
}
