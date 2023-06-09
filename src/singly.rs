#[derive(Debug)]
pub struct SinglyLinkedList<T: std::fmt::Debug + std::marker::Copy + std::fmt::Display> {
    node: NodePointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy + std::fmt::Display> {
    element: T,
    next: NodePointer<T>,
}

type NodePointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy + std::fmt::Display> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList { node: None }
    }

    fn create_node(element: T, node: NodePointer<T>) -> NodePointer<T> {
        Some(Box::new(Node {
            element,
            next: node,
        }))
    }

    pub fn add(&mut self, element: T) {
        let current_node = self.node.take();
        let new_node = Self::create_node(element, current_node);
        self.node = new_node;
        print!("Updated List: ");
        self.view_all_nodes();
    }

    pub fn remove(&mut self) {
        let current_node = self.node.take();
        let new_node = match current_node {
            Some(node) => node.next,
            None => None,
        };
        self.node = new_node;
        print!("Updated List: ");
        self.view_all_nodes();
    }

    pub fn view_all_nodes(&self) {
        let mut selected_node = &self.node;
        loop {
            match selected_node {
                Some(node) => {
                    print!("{}", node.element);
                    if node.clone().next.is_some() {
                        print!(" - ");
                    }
                    selected_node = &node.next;
                }
                None => {
                    println!("");
                    break;
                }
            };
        }
        if self.node.is_none() {
            println!("Empty");
        }
    }
}
