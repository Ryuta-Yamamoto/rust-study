use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    edges: RefCell<Vec<Rc<Edge>>>,
}

struct Edge {
    start: Weak<RefCell<Node>>,
    end: Weak<RefCell<Node>>,
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>
}


impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            edges: RefCell::new(vec![])
        }
    }
    fn add(&mut self, edge: Rc<Edge>) {
        self.edges.borrow_mut().push(edge)
    }
    fn values(&self) -> Vec<i32> {
        let mut v = vec![];
        for edge in self.edges.borrow().iter() {
            match edge.values() {
                (x, y) if y == self.value => v.push(x),
                (x, y) if x == self.value => v.push(y),
                _ => panic!()
            }
        }
        v
    }
}

impl Edge {
    fn values(&self) -> (i32, i32) {
        (
            self.start.upgrade().unwrap().borrow().value,
            self.end.upgrade().unwrap().borrow().value
        )
    }
}

impl Graph {
    fn new(values: &[i32]) -> Graph {
        Graph {
            nodes: values.iter().map(|x| Rc::new(RefCell::new(Node::new(*x)))).collect()
        }
    }
    fn connect(&mut self, start: usize, end: usize) {
        let s_node = &self.nodes[start];
        let e_node = &self.nodes[end];
        let edge = Rc::new(
            Edge {
                start: Rc::downgrade(&s_node),
                end: Rc::downgrade(&e_node),
            }
        );
        s_node.borrow_mut().add(Rc::clone(&edge));
        e_node.borrow_mut().add(Rc::clone(&edge))
    }
}


fn main() {
    let range: Vec<i32> = (0..10).collect();
    let mut graph = Graph::new(&range);
    for n in 0..10 {
        let max = std::cmp::min(10, n + 3);
        for m in n..max {
            if n != m {
                graph.connect(n, m);
            }
        }
    }

    for node in graph.nodes.iter() {
        println!("{}, {:?}", node.borrow().value, node.borrow().values())
    }
    
}
