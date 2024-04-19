
use std::iter::zip;

use crate::{N,edges};
use crate::matrix::{sub11,sub20,div20,div11_each_gen,add11_each,add22_each};
use crate::network::Client;
use crate::packet::Packet;
use crate::const_val::*;
use crate::analyzer::AnalyzeSystem;
use crate::arr2::Arr2;
pub struct VirtualSys{
    cij:f64,
    ci:[f64;N],
    qi:[f64;N],
    qij:[[f64;N];N],
    ai:[f64;N],
    aij:[[f64;N];N],
}


impl VirtualSys{
    pub fn  new(ci:[f64;N],cij:f64)-> VirtualSys{
        VirtualSys{
            ci,
            cij,
            qi: [0f64;N],
            qij:[[0f64;N];N],
            ai: [0f64;N],
            aij:[[0f64;N];N],
        }
    }
    fn update_node_load(node_load: &mut [f64;N],pac :&Packet){
        let mut acc_iter = pac.client.acc.iter();
        let mut serv_iter = pac.client.func.iter();
        for path in pac.paths[1..].iter(){
            let acc = acc_iter.next().unwrap();
            let serv = serv_iter.next().unwrap();
            node_load[path[0]]+=pac.load*acc*(serv.process_cost);
            debug!(target :"vs","path:{:?} add {}",path,pac.load*acc*(serv.process_cost));
        }
        debug!(target: "vs", "after update node load: {:?}",node_load);
    }
    fn update_link_load(link_load: &mut [[f64;N];N],pac :&Packet){
        if pac.paths.len() == 1{//static packet
            let mut s = pac.paths[0][0];
            for t in pac.paths[0][1..].iter(){
                link_load[s][*t]+=pac.load;
                s = *t;
            }
        }else{
            let mut acc_iter = pac.client.acc.iter();
            for path in &pac.paths{
                let mut s = path[0]; 
                let acc = acc_iter.next().unwrap();
                for t in path[1..].iter(){
                    link_load[s][*t]+=pac.load*(*acc);
                    s = *t;
                }
            }
        }
    }
    pub fn normal_qij (&self)->[[f64;N];N]{
        let mut res = [[f64::INFINITY;N];N];
        for Edge(s,t,_) in edges{
            res[s][t]=self.qij[s][t];
            res[t][s]=self.qij[t][s];
        }
        for i in 0..N{
            res[i][i]=0f64;
        }
        div20(&mut res,self.cij);
        debug!(target : "vs","normal_qij\n{}\n",Arr2::new("nor_qij",res));
    
        res
    }
    pub fn normal_qi(&self)->[f64;N]{
        div11_each_gen(&self.qi,&self.ci)
    }
    pub fn routing_for<'a>(&mut self,c:&'a Client,cnt:f64,t:usize,p:&[[usize;N];N],_hop_cnt:[[usize;N];N],node:(usize,usize),pac_id:usize)
        ->(Packet<'a>,Vec<Packet<'a>>)
    {
        let (l_path,s_paths) = c.generate_live_and_static_paths(p, vec![node.0,node.1]);
        let s_pacs=zip(zip(s_paths.into_iter(),c.func.iter()),c.acc.iter())
            .scan(0,|status,((path,ser),acc)|{
                assert!(path.len()!=0);

                let mut pac = Packet::new(
                    pac_id,
                    cnt*ser.merging_ratio*acc,
                    t,
                    c,
                    vec![path],
                );
                pac.set_static_stage(*status);
                *status+=1;
                Some(pac)
        }).inspect(|pac|{
            Self::update_link_load(&mut self.aij, pac);
        }).collect();
        let l_pac = Packet::new(pac_id,cnt,t,c,l_path);
        Self::update_link_load(&mut self.aij,&l_pac);
        Self::update_node_load(&mut self.ai,&l_pac);
        (l_pac,s_pacs)
    }
    // pub fn  routing_for<'a>
    //     (&mut self,client:&'a Client,load: f64,algorithm: RoutingAlgorithm,t:usize,d:[[f64;N];N],p:[[usize;N];N],hop_cnt:[[usize;N];N])
    //         ->(Packet<'a>,Vec<Packet<'a>>){
    //             // if ana.t==1 && client.client_id==0{
    //             //     info!("{}\n",load);
    //             // }
    //     //1. 用floyd 算法求出任意两点间的最短路。
        
    //     //2. 用MIN-STAR算法找到最小的节点
    //     //3. 根据找到的节点生成一系列路径
    //     let (live_path,static_paths) = client
    //         .generate_live_and_static_paths(&p,
    //             select_process_nodes_based_on(algorithm,div11_each_gen(&self.qi,&CI),d,&p,hop_cnt,client));
    //     //4. 根据这些路径分别组装live packet和static packets
    //     let mut static_packets :Vec<Packet> = Vec::with_capacity(2);
    //     let mut func_iter = client.func.iter();
    //     let mut acc_iter = client.acc.iter();
    //     let mut i=0;
    //     for path in static_paths{
    //         let acc = acc_iter.next().unwrap();
    //         let serv = func_iter.next().unwrap();
    //         let mut static_packet = Packet::new(load*serv.merging_ratio*acc,
    //                                         t,
    //                                         client,
    //                                         vec![path]);
            
            
    //         static_packet.set_static_stage(i);
    //         Self::update_link_load(&mut self.aij,&static_packet);
    //         static_packets.push(static_packet);
    //         i+=1;
            
    //     }
    //     let live_packet = Packet::new(load,t,client,live_path);
    //     Self::update_link_load(&mut self.aij,&live_packet);
    //     Self::update_node_load(&mut self.ai,&live_packet);
    //     (
    //         live_packet,
    //         static_packets,
    //     )
    // }
    pub fn  tic_toc(&mut self,ana: &mut AnalyzeSystem){
        debug!(target : "vs","t:{}\n{:?}\n{}\n",ana.t,self.qi,Arr2::new("qij",self.qij));
        add11_each(&mut self.qi,&self.ai);
        add22_each(&mut self.qij,&self.aij);
        debug!(target : "vs","after add :\n{:?}\n{}\n",self.qi,Arr2::new("qij",self.qij));
        
        sub11(&mut self.qi,&self.ci);
        sub20(&mut self.qij,self.cij);
        self.ai=[0f64;N];
        self.aij=[[0f64;N];N];
    }
}
