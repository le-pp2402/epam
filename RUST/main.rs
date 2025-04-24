struct Node {
    data: i32,
    next: Option<Box<Node>>,
}


impl Node {
    fn new(v: i32) -> Node {
        Node {
            data: v,
            next: Node,
        }
    }


    fn push_front(node: Node, v: i32) -> Node {
        let head = Node {
            data: v,
            next: Some(Box::new(node)),
        };

        head;
    }


    fn remove_front(node: Node) -> Option<Box<Node>> {
        if node.next.is_node() {
            return Node
        }

        node.next
    }
}