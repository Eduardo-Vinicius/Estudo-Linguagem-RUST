

fn e1_Ordernar(mut num:Vec<i32>) -> Vec<i32>{
    let mut i = 0;
    let mut j = 0;
    while i < num.len(){
        while j < num.len(){
            if num[j] < num[i]{
                let aux = num[i];
                num[i] = num[j];
                num[j] = aux;
            }
            j += 1;
        }
        i += 1;
    } 
    return num;
}