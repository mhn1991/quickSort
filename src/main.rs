mod sort;
use sort::List;
fn main() {
    let list = vec![10, 7, 8, 9, 1, 5];
    let mut m = List::new_list(list);
    let end = m.get_len();
    m.sort(0,end-1);
    m.write();
}
