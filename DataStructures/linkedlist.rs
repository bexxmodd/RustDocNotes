use std::cell::RefCell;
use std::cell::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clode)]
struct Node {
    value: String;
    next: SingleLink,
}

impl Node {
    // create a new node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    // initialize empty log
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
}