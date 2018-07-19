//mod sort;
//use sort::List;
mod sortpointer;
use sortpointer::List;
fn main() {
    let mut list = vec![10, 7, 8, 9, 1, 5];/*
    let mut m = List::new_list(list);
    let end = m.get_len();
    m.sort(0,end-1);
    m.write();
     */
    let mut m = List::new_list(&mut list);
    let end = m.get_len();
    m.sort(0,end-1);
    m.write();
}
