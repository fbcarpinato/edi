use crate::rope::Rope;

pub struct Editor {
    pub data: Rope,
}

impl Editor {
    pub fn new(content: &str) -> Editor {
        Editor {
            data: Rope::from_str(content),
        }
    }
}
