use crate::database::Databases;
#[allow(non_camel_case_types)]
#[derive(Copy,Clone)]
pub enum RoutingAlgorithm{
    L2S,
    S2L,
    DI_DCNC,
}
use std::fmt;
use std::iter::zip;
impl fmt::Display for RoutingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoutingAlgorithm::L2S => write!(f, "L2S"),
            RoutingAlgorithm::S2L => write!(f, "S2L"),
            RoutingAlgorithm::DI_DCNC => write!(f, "DI_DCNC"),
        }
    }
}

use crate::network::Client;
pub fn select_nodes_based_on<const N:usize>
    (alg:RoutingAlgorithm,comput: [f64;N],d:[[f64;N];N],p:&[[usize;N];N],hop_cnt:[[usize;N];N],c:&Client,dbs:&Databases)
    ->Vec<(usize,usize)>{
    debug!(target : "vs","select process nodes with :{:?}\n{}\n{}",
    comput,
        Arr2::new("node cost",d),
        Arr2::new("next pac",p.clone()),
    );
    match alg{
        RoutingAlgorithm::L2S => l2s(c,dbs),
        RoutingAlgorithm::S2L => s2l(comput,d,p,hop_cnt,c,dbs),
        RoutingAlgorithm::DI_DCNC =>di_dcnc(comput,d,hop_cnt,c,dbs),
    }
}
use crate::arr2::Arr2;
// use rand::prelude::*;
fn s2l<const N:usize>(comput:[f64;N],d:[[f64;N];N],_next_pac:&[[usize;N];N],hop:[[usize;N];N],c:&Client,dbs:&Databases)
    ->Vec<(usize,usize)>{
    let mut proce=[[0f64;N];N];
    let mut min_cost = f64::INFINITY;
    for i in 0..N{
        for j in 0..N{
            proce[i][j]=
                (d[c.s][i] +comput[i]) *c.acc[0]+
                (d[i][j]     +comput[j]) *c.acc[1]+
                (d[j][c.t])            *c.acc[2];
            if proce[i][j]<min_cost{
                min_cost = proce[i][j];
            }
        }
    }
    let mut res :Vec<(usize,usize,usize)> = Vec::new();
    for i in 0..N{
        for j in 0..N{
            if proce[i][j]==min_cost{
                let now_hop_cnt = 
                    hop[c.s][i]+
                    hop[i][j]+
                    hop[j][c.t];
                res.push((i,j,now_hop_cnt));
            }
        }
    }
    res.sort_unstable_by(|a,b| a.2.cmp(&b.2));
    res.into_iter().map(|(i,j,_)|(i,j)).collect()
}
fn l2s(c:&Client,db:&Databases)->Vec<(usize,usize)>{
    let i = db.get_db_pos(c.func[0].db_ind(),c.s);
    let j = db.get_db_pos(c.func[1].db_ind(),i);
    vec![(i,j)]
}
fn di_dcnc<const N:usize>(comput:[f64;N],dist:[[f64;N];N],hop:[[usize;N];N],c:&Client,dbs:&Databases)
    ->Vec<(usize,usize)>{
    let mut proce=[[0f64;N];N];
    let mut min_cost = f64::INFINITY;
    for i in 0..N{
        for j in 0..N{
            proce[i][j]=
                (dist[c.s][i]+dbs.get_db_cost(c.func[0].db_ind(),i)*c.func[0].merging_ratio()+comput[i]*c.func[0].process_cost())*c.acc[0]+
                (dist[i][j]    +dbs.get_db_cost(c.func[1].db_ind(),j)*c.func[1].merging_ratio()+comput[j]*c.func[1].process_cost())*c.acc[1]+
                (dist[j][c.t])*c.acc[2];
            if proce[i][j]<min_cost{
                min_cost = proce[i][j];
            }
        }
    }
    let mut res :Vec<(usize,usize,usize)> = Vec::new();
    for i in 0..N{
        for j in 0..N{
            if proce[i][j]==min_cost{
                let now_hop_cnt = 
                    hop[c.s][i]+
                    hop[i][j]+
                    hop[j][c.t]+
                    hop[dbs.get_db_pos(c.func[0].db_ind(),i)][i]+
                    hop[dbs.get_db_pos(c.func[1].db_ind(),j)][j];
                res.push((i,j,now_hop_cnt));
            }
        }
    }
    res.sort_unstable_by(|a,b| a.2.cmp(&b.2));
    res.into_iter().map(|(i,j,_)|(i,j)).collect()
}use crate::graph::{generate_paths,generate_single_path};
/**
 * # 功能
 * 根据floyd算法计算出的下一跳数组以及MIN-STAR算法处理出的一组处理节点来生成一组路径，包括一个live packet的路径和一组static packets的路径
 * 
 */
pub fn generate_live_and_static_paths<const N:usize>
    (c:&Client,p:&[[usize;N];N],mut nodes: Vec<usize>,dbs:&Databases)
    ->(Vec<Vec<usize>>,Vec<Vec<usize>>){
    
    nodes.push(c.t);
    let static_paths:Vec<Vec<usize>> = zip(nodes.iter(),c.func.iter())
        .map(|(node,serv)|{
        generate_single_path(p,dbs.get_db_pos(serv.db_ind(),*node),*node)
    }).collect();
    assert_eq!(nodes.len(),static_paths.len()+1);
    (generate_paths(p,c.s,nodes),static_paths)
}
