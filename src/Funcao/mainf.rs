
fn mainf(){
    //Primitive
    crate::Funcao::Primitive::e1_Declaracao();
    crate::Funcao::Primitive::e1_OperadoresMatematicos();
    crate::Funcao::Primitive::e1_Vetor();

    //Procedures
    let x1: f64 = crate::Funcao::Procedure::double(20.0);
    let x2: f64 = crate::Funcao::Procedure::somar(30.0, 40.0);
    let x3: f64 = crate::Funcao::Procedure::subtrair(40.0);
    println!("{:.2}", x3)

    let x4: i64 = crate::Funcao::Procedure::e1_area_retangulo(20,10);
    let x5: 
}