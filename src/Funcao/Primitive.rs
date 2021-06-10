
fn e1_OperadoresMatematicos(){
    //
    let x1 = 10 + 10;
    let x2 = 10 - 10;
    let x3 = 10 * 10;
    let x4 = 10 / 10;
    let x5 = 10 % 10;

    //
    let x3 += 10;
    let x3 -= 05;
    let x3 *= 02;
    let x3 /= 02;

    //
    let x6 = x1 + x2;
    let x7 = x1 + x2 * x3;
    let x8 = (x1 + x2) * x3;

}

fn e2_OperadoresRelacionais(){
    let x1:bool = 10 > 5;
    let x2:bool = 10 < 5;
    let x3:bool = 10 >= 5;
    let x4:bool = 10 <= 5;
    let x5:bool = 10 == 5;
    let x6:bool = 10 != 5;

    let x7 = 10;
    let x8 = 5;

    let x9:bool = x7 > 10;
    let x10:bool = x7 > x8;

}

fn e3_OperadoresLogicos(){
    let x1:bool = true && true;
    let x2:bool = true && false;
    let x3:bool = true || true;
    let x4:bool = true || false;
    let x5:bool = false || false;
    let x6:bool = !false;
    let x7:bool = !true;

    let x8:bool = 10 > 5  && 20 < 10;
    let x9:bool = 10 > 5  || 20 < 10;

    let x10 = 10;
    let x11 = 5;

    let x12:bool = x10 > 5 && x11 < x10;
    let x13:bool = x10 > 5 || x11 < x10;
 }
