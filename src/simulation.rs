use crate::corealgorithm::{select_nodes_based_on, RoutingAlgorithm};
use crate::const_val::*;
use crate::database::Databases;
use crate::network::{Client,Network};
use crate::graph::floyd;
use crate::matrix::mul10_gen;
use crate::analyzer::AnalyzeSystem;
// use graph::{EdgeType,create_graph};
use crate::virtualsystem::VirtualSys;
use rand_distr::{Poisson,Distribution};
macro_rules! i{
    ($c:literal) => {
        (($c as u8)+8-('A' as u8)) as usize
    };
}
#[derive(Debug)]
pub struct SimulateResult{
    pub ave_delay : f64,
}
pub fn simulate_alpha(node_ratio:f64,link_ratio:f64,rout_alg:RoutingAlgorithm)->bool{
    assert!(node_ratio<=1f64 &&node_ratio>=0f64&&link_ratio<=1f64&&link_ratio>=0f64);
    simulate(4f64,rout_alg,20f64,node_ratio,link_ratio).ave_delay<=20f64
}
pub fn simulate_replica_index(replica_ind:usize,_rout_alg:RoutingAlgorithm){
    assert!(replica_ind<=N);

}
pub fn simulate_average_delay(lambda:f64,rout_alg:RoutingAlgorithm)->SimulateResult{
    simulate(lambda,rout_alg,100f64,1f64,1f64)
}
fn simulate(lambda :f64, rout_alg:RoutingAlgorithm,delay_limit:f64,node_ratio:f64,link_ratio:f64)->SimulateResult{
    debug!("Start simulation!");
    let lambda  = lambda;
    let mut vs : VirtualSys = VirtualSys::new(mul10_gen(&CI,node_ratio), CIJ*link_ratio);
    let mut net: Network = Network::new(mul10_gen(&CI,node_ratio),[[CIJ*link_ratio;N];N]);
    let poi=Poisson::new(lambda).unwrap();

    let c_list:[Client;4]=[
        Client::new(0,[service0,service1],i!('E'),i!('H'),[1f64,1f64,2f64]),
        Client::new(1,[service2,service3],i!('F'),i!('G'),[1f64,1f64,0.5f64]),
        Client::new(2,[service4,service5],i!('G'),i!('F'),[1f64,1f64,3f64]),
        Client::new(3,[service6,service7],i!('H'),i!('E'),[1f64,0.5f64,1f64/6f64]),
    ];
    let mut dbs = Databases::new([vec![0],vec![1],vec![2],vec![3],vec![4],vec![5],vec![6],vec![7]]);
    let mut pac_id = 1..;
    let mut ana = AnalyzeSystem::default();
    loop{
        info!(target : "net","<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<tic toc!, at time slot {}>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>",ana.t);
        let (d,p,hop_cnt)=floyd(vs.normal_qij());
        dbs.preprocess_nearest(&d);
        for c in &c_list{
            let load = (poi.sample(&mut rand::thread_rng()) )as usize;
            let nodes=select_nodes_based_on(rout_alg,vs.normal_qi(),d,&p,hop_cnt,c,&dbs);
            for i in 0..load{
                let t = ana.t;
                net.arrive(&mut ana,vs.routing_for(&c,1.0,t,&p,hop_cnt,nodes[i%nodes.len()],pac_id.next().unwrap(),&dbs));
            }
        }
        //滴答~
        ana.tic_toc();
        vs.tic_toc(&mut ana);
        net.tic_toc(&mut ana);
        if ana.t>TIME_LIMIT{
            break;
        }
        
        debug!(target: "ana","{}",ana.report());
        if ana.average_delay()>delay_limit {
            break;
        }
    }
    SimulateResult{
        ave_delay: ana.average_delay(),
    }
}