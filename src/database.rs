pub struct DataBase{
    database_pos:Vec<usize>,
}
use crate::const_val::*;
use std::cmp;
impl DataBase{
    pub fn nearest(&self,cost:[[f64;N];N]){
        let min_cost = f64::INFINITY;
        let pos = N;
        self.database_pos.iter().reduce(|status,pos|{
            cmp::min(status,pos)
        }).unwrap();

    }
}