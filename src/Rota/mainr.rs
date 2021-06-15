
pub fn mainr(){
    //Conditional
    let x0 = crate::Rota::Conditional::e1_IF_Aprovado(5.0);
    let x1 = crate::Rota::Conditional::e2_IF_Aprovado(5.0,1);
    let x2 = crate::Rota::Conditional::e3_Switch_Avaliacao("A"); 
    let x3 = crate::Rota::Conditional::e4_Switch_Verdadeiro();

    //Looping
    let x4 = crate::Rota::Looping::e2_WHILE_SomarNumeros(5);
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x5 = crate::Rota::Looping::e3_FOR_FiltrarPares(vec);
    let x6 = crate::Rota::Looping::e4_CONTINUE_GerarSeqeunciaPar(5);
    let mut vec2 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x7 = crate::Rota::Looping::e5_BREAK_TodosPares(vec2);
    let x8 = crate::Rota::Looping::e6_WHILE_ProximaRaizInteira(5.0);

    //CompoundLooping
    let mut vec3 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x9 = crate::Rota::CompoundLooping::e1_Ordernar(vec3);

    //CompoundConditional
    let x10 = crate::Rota::CompoundConditional::e1_RecomendacaoMusica("Brasil","MPB");
    println!("{}",x10);
}