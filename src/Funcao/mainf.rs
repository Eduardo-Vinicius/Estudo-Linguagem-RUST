
pub fn mainf(){
    //Primitive
    crate::Funcao::Primitive::e1_OperadoresMatematicos();
    crate::Funcao::Primitive::e2_OperadoresRelacionais();
    crate::Funcao::Primitive::e3_OperadoresLogicos();

    //Procedures
    let x1: f64 = crate::Funcao::Procedure::double(20.0);
    let x2: f64 = crate::Funcao::Procedure::somar(30.0, 40.0);
    let x3: f64 = crate::Funcao::Procedure::subtrair(40.0,30.0);

    let x4: i64 = crate::Funcao::Procedure::e1_area_retangulo(20,10);
    let x5: f64 = crate::Funcao::Procedure::e2_media(5.5, 8.3, 10.0);
    let x6: f64 = crate::Funcao::Procedure::e3_hipotenusa(3, 4);

    //Recursive
    let x7 = crate::Funcao::Recursive::e2_Fatorial_Interativo(5.0);

    //highlevel
    crate::Funcao::HighLevel::e1_FuncoesParaNumeros();
    crate::Funcao::HighLevel::e2_FuncoesParaTexto();
    crate::Funcao::HighLevel::e3_FuncoesParaTiposRecursivos();
    crate::Funcao::HighLevel::e4_FuncoesParaConversao();

    //HighOrderFunction
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x8 = crate::Funcao::HighOrderFunction::e1_ComoParametro_Filter(vec,crate::Funcao::HighOrderFunction::compara);

    //CompoundProcedure
    crate::Funcao::CompoundProcedures::mainProcedure();
}

