#![allow(unused)]

pub fn e1_Declaracao(){
    //
    let x0 = true;
    let x1 = 10;
    let x2 = 10.5;
    let x3 = "olá";
    let x4 = 'a';
    println!("{},{},{},{},{}",x0,x1,x2,x3,x4);

    let x5:bool = true;
    let x6:i32 = -10;
    let x7:u32 = 10;
    let x8:f32 = 10.5;
    let x9:String = "olá".to_string();
    let x10:char = 'a';
    println!("{},{},{},{},{},{}",x5,x6,x7,x8,x9,x10);

    //
    let x12 = x6;
    let x13 = x6 + 5;

    println!("{},{}",x12,x13);
    
}

pub fn e2_Alteracao(){
    //
    let mut x1 = 10;
    println!("{}",x1);

    x1 = 15;
    println!("{}",x1);
    //
    let x2 = "oie";
    println!("{}",x2);

    let x2 = "olá";
    println!("{}",x2);

    //
    let mut x3 = 10;
    println!("{}",x3);
    
    x3 = x1 + 5;
    println!("{}",x3);
    


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