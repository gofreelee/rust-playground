[一篇著名的教人写链表的文章](https://rust-unofficial.github.io/too-many-lists/#learn-rust-with-entirely-too-many-linked-lists)

# 第一种链表

考虑下可能的链表布局：
list = empty | data list; 
文章中提出了三种布局：

## layout1

enum List{

    Empty,
    Elem(i32, Box<List>),

}

这种布局很不优雅。举例， 一个链表如果是1 -> 2 -> null, 那么对应的是
[] = Stack
() = Heap

[1, ptr] -> (2, ptr) -> (Empty, *junk*)
可见，由于List:: Elem实际是一个指向 List类型的指针， 这样在链表的结尾部分，仍要有一个指针指向一个List， 但这个list是 list::empty

## layout2

enum List{

    Empty,
    ElemAtEnd(i32),
    ElemNotAtEnd(i32, Box<List>),

}
这种布局也有很多缺点。依然以1 -> 2 -> null为例
[] = Stack
() = Heap
[1, ptr] -> (2, *null*)
这种布局避免了指向一个没有必要的节点。但是浪费了很多空间， 这个是一个著名的rust的空指针优化。
enum Demo{

    A,
    B(...)

}
以这个Demo 枚举为例， 只有 A， B两个类型， 那么编译器不用给enmu 加tag了， 当Demo对应的那块数据是全0的时候，因为空指针不被允许， 自然对应的就是A类型
但是layout2 却是有3个类型，就没有办法使用空指针优化了。

## layout3

enum List{

    Empty,
    Elem(Box<Node>),

}

struct Node{

    data: i32,
    next: List,

}

[List(ptr) -> Node(1, next)] -> [List(ptr) -> Node(2, empty)]

既不用指向一个没必要的尾节点，也可以享受空指针优化。
