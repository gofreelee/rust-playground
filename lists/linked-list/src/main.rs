mod lists;
#[derive(Debug)]
struct node {
    data: i32,
    next: Option<Box<node>>,
}
fn main() {
    println!("Hello, world!");
    let mut a = Box::new(12);
    let mut b = Box::new(132);
    let c = a;
    let mut node1 = node {
        data: 23,
        next: None,
    };
    let mut node2 = Box::new(node {
        data: 24,
        next: None,
    });
    let mut node3 = Box::new(node {
        data: 25,
        next: None,
    });
    node1.next = Some(node2);
    //a = b;
    println!("{:#?}", node1.next);
    node3.next = node1.next;

    node1.next = Some(node3);
    println!("{:#?}", node1);
}
