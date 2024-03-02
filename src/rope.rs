enum RopeNode {
    Leaf {
        value: String,
    },
    Concatenate {
        left: Box<RopeNode>,
        right: Box<RopeNode>,
    },
}

impl RopeNode {
    fn to_string(&self, result: &mut String) {
        match self {
            RopeNode::Leaf { value } => {
                result.push_str(value);
            }
            RopeNode::Concatenate { left, right } => {
                left.to_string(result);
                right.to_string(result);
            }
        }
    }
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
            len: 0,
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
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.len);
        self.root.to_string(&mut result);
        result
    }
}
