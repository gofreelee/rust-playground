use std::mem;
struct Node {
    data: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn New() -> Self {
        return List { head: Link::Empty };
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let res;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                res = None;
            }
            Link::More(node) => {
                res = Some(node.data);
                self.head = node.next;
            }
        };
        return res;
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        loop {
            match cur_link {
                Link::Empty => {
                    break;
                }
                Link::More(mut boxed_node) => {
                    cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::New();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}