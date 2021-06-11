
fn e2_ComoParametro_Filter(mut l:LinkedList,Box<fn(mut n:i32,mut t:bool)> filtro){
    let mut novo:LinkedList<i32> = LinkedList::new();
    for_each item in l {
        if filtro(item){
            novo.push_back(item);
        }
    }
    return novo;
}