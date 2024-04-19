#[allow(non_camel_case_types)]
#[derive(Copy,Clone)]
pub enum RoutingAlgorithm{
    L2S,
    S2L,
    DI_DCNC,
}
use std::fmt;
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
    (alg:RoutingAlgorithm,comput: [f64;N],d:[[f64;N];N],p:&[[usize;N];N],hop_cnt:[[usize;N];N],c:&Client)
    ->Vec<(usize,usize)>{
    debug!(target : "vs","select process nodes with :{:?}\n{}\n{}",
    comput,
        Arr2::new("node cost",d),
        Arr2::new("next pac",p.clone()),
    );
    match alg{
        RoutingAlgorithm::L2S => l2s(c),
        RoutingAlgorithm::S2L => s2l(comput,d,p,hop_cnt,c),
        RoutingAlgorithm::DI_DCNC =>di_dcnc(comput,d,hop_cnt,c),
    }
}
use crate::arr2::Arr2;
// use rand::prelude::*;
fn s2l<const N:usize>(comput:[f64;N],d:[[f64;N];N],_next_pac:&[[usize;N];N],hop:[[usize;N];N],c:&Client)
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
// fn s2l<const N:usize>
//     (node_cost: [f64;N],path_cost:[[f64;N];N],_next_pac:&[[usize;N];N],hop_cnt:[[usize;N];N],client:&Client)
//     ->Vec<usize>{
//     let mut res:Vec<usize> = Vec::with_capacity(0);
//     let mut min_cost = f64::INFINITY;
//     let mut min_hop_cnt = f64::INFINITY;
//     let mut rng = rand::thread_rng();
//     for i in 0..N{
//         for j in 0..N{
//             let now_cost = 
//                 path_cost[client.s()][i]+
//                 node_cost[i]+
//                 path_cost[i][j]*client.acc[1]+
//                 node_cost[j]*client.acc[1]+
//                 path_cost[j][client.t()]*client.acc[2];
//             let now_hop_cnt = 
//                 hop_cnt[client.s()][i]as f64*client.acc[0]+
//                 hop_cnt[i][j]as f64*client.acc[1]+
//                 hop_cnt[j][client.t()]as f64*client.acc[2]+
//                 rng.gen::<f64>()/100.0;
//             let mut flag=false;
//             if min_cost > now_cost{
//                 flag=true;
//             }else if min_cost == now_cost{
//                 if now_hop_cnt<min_hop_cnt{
//                     flag=true;
//                 }
//             }
//             if flag==true{
//                 min_cost = now_cost;
//                 min_hop_cnt=now_hop_cnt;
//                 res = vec![i,j];
//             }
//         }
//     }
//     // let live_path = generate_single_path(&next_pac, client.s(), client.t());
//     // for i in &live_path{
//     //     for j in &live_path{
//     //         let now_cost = 
//     //             path_cost[client.func[0].database_id][*i]*client.func[0].merging_ratio+
//     //             path_cost[client.func[1].database_id][*j]*client.func[1].merging_ratio*client.func[0].scaling_factor+
//     //             node_cost[*i]*client.func[0].process_cost+
//     //             node_cost[*j]*client.func[0].scaling_factor*client.func[1].process_cost;
//     //         // let now_cost = 
//     //         //     path_cost[client.s()][i]+
//     //         //     node_cost[i]+
//     //         //     path_cost[i][j]*client.acc[1]+
//     //         //     node_cost[j]*client.acc[1]+
//     //         //     path_cost[j][client.t()]*client.acc[2];
//     //         if min_cost > now_cost{
//     //             min_cost = now_cost;
//     //             res = vec![*i,*j];
//     //         }
//     //     }
//     // }
//     // if res.is_empty(){
//     //     println!("{:?}\n",node_cost);
//     //     println!("{}\n",Arr2::new("path_cost".to_string(),path_cost));
//     //     panic!("process nodes are empty!!!");
//     // }
//     res    
// }
fn l2s(c:&Client)->Vec<(usize,usize)>{
    vec![(c.func[0].database_id,c.func[1].database_id)]
}
// fn l2s
//     (client:&Client)->Vec<usize>{
//     vec![client.func[0].database_id,client.func[1].database_id]
// }
fn di_dcnc<const N:usize>(comput:[f64;N],dist:[[f64;N];N],hop:[[usize;N];N],c:&Client)
    ->Vec<(usize,usize)>{
    let mut proce=[[0f64;N];N];
    let mut min_cost = f64::INFINITY;
    for i in 0..N{
        for j in 0..N{
            proce[i][j]=
                (dist[c.s][i]+dist[c.func[0].database_id][i]*c.func[0].merging_ratio+comput[i]*c.func[0].process_cost)*c.acc[0]+
                (dist[i][j]    +dist[c.func[1].database_id][j]*c.func[1].merging_ratio+comput[j]*c.func[1].process_cost)*c.acc[1]+
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
                    hop[c.func[0].database_id][i]+
                    hop[c.func[1].database_id][j];
                res.push((i,j,now_hop_cnt));
            }
        }
    }
    res.sort_unstable_by(|a,b| a.2.cmp(&b.2));
    res.into_iter().map(|(i,j,_)|(i,j)).collect()
}
// fn di_dcnc<const N:usize>
// (node_cost: [f64;N],path_cost:[[f64;N];N],hop_cnt:[[usize;N];N],client:&Client)
// ->Vec<usize>{
//     let mut res:Vec<usize>=Vec::with_capacity(0);
//     let mut min_cost = f64::INFINITY;
//     let mut rng = rand::thread_rng();
//     let mut min_hop_cnt=f64::INFINITY;
//     for i in 0..N{
//         for j in 0..N{
//             let now_cost = 
//                 (path_cost[client.s()][i]+path_cost[client.func[0].database_id][i]*client.func[0].merging_ratio+node_cost[i]*client.func[0].process_cost)*client.acc[0]+
//                 (path_cost[i][j]         +path_cost[client.func[1].database_id][j]*client.func[1].merging_ratio+node_cost[j]*client.func[1].process_cost)*client.acc[1]+
//                 (path_cost[j][client.t()])*client.acc[2];
//             debug!(target:"vs","min_cost ={:.4},cost at {i} {j} is{:.4}\n",min_cost,now_cost);
//             let now_hop_cnt = 
//                 (hop_cnt[client.s()][i] as f64  +hop_cnt[client.func[0].database_id][i] as f64 * client.func[0].merging_ratio)*client.acc[0]+
//                 (hop_cnt[i][j]as f64            +hop_cnt[client.func[1].database_id][j] as f64 * client.func[1].merging_ratio)*client.acc[1]+
//                 (hop_cnt[j][client.t()] as f64  )*client.acc[2]+
//                 rng.gen::<f64>()/100.0;
//             let mut flag=false;
//             if now_cost<min_cost{
//                 flag=true;
//             }else if now_cost==min_cost{
//                 if now_hop_cnt<min_hop_cnt{
//                     flag=true;
//                 }
//             }
//             if flag==true{
//                 min_cost = now_cost;
//                 min_hop_cnt = now_hop_cnt;
//                 res = vec![i,j];
//             }
//         }
//     }
//     debug!(target:"vs","processing at {} {}",res[0],res[1]);
//     res
// }

