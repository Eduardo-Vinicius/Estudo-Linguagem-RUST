#![allow(unused)]

pub fn e1_Declaracao(){
    //
    let x0 = true;
    let x1 = 10;
    let x2 = 10.5;
    let x3 = "olá";
    let x4 = 'a';
    
    let x5:bool = true;
    let x6:i32 = -10;
    let x7:u32 = 10;
    let x9:f32 = 10.5;
    let x10:String = "olá".to_string();
    let x11:char = 'a';

    //
    let x12 = x6;
    let x13 = x6 + 5;

    println!("{}",x13);
    
}

pub fn e2_Alteracao(){
    //
    let mut x1 = 10;
    x1 = 15;

    //
    let x2 = "oie";
    let x2 = "olá";
    
    //
    let mut x3 = 10;
    x3 = x1 + 5;
    


}

pub fn e3_Conversao(){
    let x = "10.5";
    //
    let x1 = x.parse::<u32>().unwrap();
    let x2 = x.parse::<bool>().unwrap();
    let x3 = x.parse::<f32>().unwrap();
    let x4 = x.parse::<char>().unwrap();
    let x5 = x.to_string();

    
}

pub fn e4_Coercion(){
    //
    let x1:i32 = 10;
    let x2:u32 = x1 as u32;

    //
    let x3:f32 = x2 as f32;

    let mut x4:String = "x = ".to_string();
    x4 = format!("x = {}",x1);
    x4 = format!("x = {} e {}",x1,x2);
}

pub fn e5_Inferencia(){
    //
    let x1 = 10_i32;
    let x2 = 10_u32;
    let x3 = 10_f32;
    let x4 = "olá".to_string();
}