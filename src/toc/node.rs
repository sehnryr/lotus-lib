use std::{cell::RefCell, rc::Rc};

pub trait Node {
    fn name(&self) -> &String;
    fn parent(&self) -> Option<Rc<RefCell<dyn Node>>>;
    fn toc_offset(&self) -> i64;
    fn path(&self) -> String;
}
