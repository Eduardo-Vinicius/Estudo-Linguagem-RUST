fn e1_ComoParametro_Filter(mut l:Vec<i32>,f:fn(i32) -> bool) -> LinkedList<i32>{
    let mut novo:LinkedList<i32> = LinkedList::new();
    for item in &l {
        if f(*item){
            novo.push_back(*item);
        }
    }
    return novo;
}
fn compara(n: i32) -> bool{
    if n >= 4 {
        return true;
    }
    else{
        return false;
    }
}