use std::rc::Rc;

pub trait Node {
    fn name(&self) -> &String;
    fn parent(&self) -> Option<Rc<dyn Node>>;
    fn toc_offset(&self) -> u64;
    fn path(&self) -> String;
}