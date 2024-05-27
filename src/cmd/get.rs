use crate::{Connection, Db, Frame, Parse};

#[derive(Debug)]
pub struct Get {
    key: String,
}
impl Get {
    pub fn new(key: impl ToString) -> Get  {

    }
}