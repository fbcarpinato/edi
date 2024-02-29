enum RopeNode {
    Leaf {
        value: String,
    },
    Concatenate {
        left: Box<RopeNode>,
        right: Box<RopeNode>,
    },
}

pub struct Rope {
    root: RopeNode,
    len: usize,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            root: RopeNode::Leaf {
                value: String::new(),
            },
            len: 0
        }
    }

    pub fn from_str(s: &str) -> Self {
        Rope {
            root: RopeNode::Leaf {
                value: s.to_string(),
            },
            len: s.len(),
        }
    }
}
