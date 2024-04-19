use crate::const_val::*;
use derive_new::new;
use std::iter::zip;
#[derive(new,Default)]
struct DB{
    db_pos:Vec<usize>,
    #[new(value="[N;N]")]
    nearest_to:[usize;N],
    #[new(value="[f64::INFINITY;N]")]
    min_cost:[f64;N],
}
impl DB{
    fn preprocess_nearest(&mut self,cost:&[[f64;N];N]){
        self.min_cost = [f64::INFINITY;N];
        self.db_pos.iter().for_each(|db|{
            zip(cost[*db].iter(), zip(self.nearest_to.iter_mut(),self.min_cost.iter_mut()))
                .for_each(|(now_cost,(min_pos,min_cost))|{
                    if *now_cost<*min_cost{
                        *min_pos=*db;
                        *min_cost=*now_cost;
                    }
                })
        });
    }

}
#[derive(Default)]
pub struct Databases{
    dbs:[DB;M],
}
impl Databases{
    pub fn new(arr:[Vec<usize>;M])->Self{

        let mut res = Databases::default();
        for i in 0..M{
            res.dbs[i] = DB::new(arr[i].clone());
        }
        res
    }
    pub fn preprocess_nearest(&mut self,cost:&[[f64;N];N]){
        self.dbs.iter_mut().for_each(|db|{
            db.preprocess_nearest(cost);
        });
    }
    pub fn get_db_pos(&self,db_ind:usize,node_pos:usize)->usize{
        self.dbs[db_ind].nearest_to[node_pos]
    }
    pub fn get_db_cost(&self,db_ind:usize,node_pos:usize)->f64{
        self.dbs[db_ind].min_cost[node_pos]
    }
}
// #[test]
// fn test_db(){
//     let mut db = DataBase::new(vec![2,1]);
//     db.preprocess_nearest([
//         [0.0,0.1,0.2],
//         [0.1,0.0,0.15],
//         [0.2,0.15,0.0],
//     ]);
//     assert_eq!(db.nearest_pos,[1,1,2]);
//     assert_eq!(db.min_cost,[0.1,0.0,0.0]);
// }