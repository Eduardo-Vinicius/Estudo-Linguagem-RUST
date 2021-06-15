use std::collections::LinkedList;
pub fn e1_loopexpression(){
    //loop { println!("I live."); } //loop infinito
    //loop { println!("I live."); break;} //loop com uma volta
}

pub fn e2_WHILE_SomarNumeros(fim:i32) -> i32{
    let mut soma = 0;
    let i = 0;
    while i <= fim {
        soma += 1; 
    }
    return soma;
}

pub fn e3_FOR_FiltrarPares(mut num:Vec<i32>) -> LinkedList<i32>{
    let mut pares:LinkedList<i32> = LinkedList::new();
    for item in &num{
        if item % 2 == 0{
            pares.push_back(*item);
        }
    }
    return pares;
}

pub fn e4_CONTINUE_GerarSeqeunciaPar(fim:i32) -> LinkedList<i32>{
    let mut seq:LinkedList<i32> = LinkedList::new();
    let i = 0;
    while i <= fim {
        if i % 2 != 0{
            continue;
        }
        seq.push_back(i);
    } 
    return seq; 
}

pub fn e5_BREAK_TodosPares(num:Vec<i32>) -> bool{
    let mut pares:bool = true;
    for item in num {
        if item % 2 != 0 {
            pares = false;
            break;
        }
    }
    return pares;
}

pub fn e6_WHILE_ProximaRaizInteira(mut num:f32) -> f32{
    let mut resto:f32 = 1.0;
    while resto != 0.0{
        num + 1.0;
        resto = num.sqrt() % 1.0;
    }
    return num;
}
