use derive_new::new;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::fmt;
use std::iter::zip;
use crate::const_val::*;
use crate::{N,edges};
use crate::analyzer::AnalyzeSystem;
use crate::packet::{Packet,PacketPair};
use crate::arr2::Arr2;

#[derive(Default)]
pub struct Node<'a,'b,'c>{
    pub unpaired_hash_map: HashMap<(usize,usize),Packet<'a>>,
    pub paired_queue: BinaryHeap<PacketPair<'b>>,
    pub waiting_list: Vec<Packet<'c>>,
    pub cap: f64,
    pub rem: Option<f64>,
}
impl Node<'_,'_,'_>{
    pub fn new(c: f64)->Self{
        Self{
            unpaired_hash_map: HashMap::new(),
            paired_queue: BinaryHeap::new(),
            waiting_list: Vec::new(),
            cap : c,
            rem : None,
        }
    }
}
impl <'a> Node<'a,'a,'a>{

    fn receive(&mut self, pac:Packet<'a>){
        #[cfg(feature = "check_variable")]
        assert!(pac.status().0==true);
        let key = (pac.pac_id,pac.client.func[pac.stage()].service_id);
        match self.unpaired_hash_map.remove(&key){
            None =>{
                debug!(target : "net", "to be coupled {}\n",pac);
                self.unpaired_hash_map.insert(key,pac);
            }
            Some(counter_parts) =>{
                debug!(target : "net" , "coupled!!!{}\n",pac);
                match pac.is_live(){
                    false => {
                        
                        self.paired_queue.push(PacketPair(counter_parts,pac));
                    }
                    true =>{
                        self.paired_queue.push(PacketPair(pac,counter_parts));
                    }
                }
            }
        }
    }
    pub fn receive_later(&mut self,pac:Packet<'a>){
        self.waiting_list.push(pac);
    }
    pub fn flush(&mut self){
        while let Some(pac) = self.waiting_list.pop(){
            self.receive(pac)
        }
    }
}
#[derive(Default)]
pub struct Link<'a>{
    pub que: BinaryHeap<Packet<'a>>,
    pub waiting_list: Vec<Packet<'a>>,
    pub cap: f64,
    pub rem: Option<f64>,
}
impl <'a>Link<'a>{
    pub fn new(c:f64)->Self{
        Self{
            que: BinaryHeap::new(),
            waiting_list: Vec::new(),
            cap :c,
            rem :None,
        }
    }
    fn receive(& mut self, pac :Packet<'a>){
        self.que.push(pac);
    }
    pub fn receive_later(&mut self, pac:Packet<'a>){
        self.waiting_list.push(pac);
    }
    pub fn flush(&mut self){
        while let Some(pac) = self.waiting_list.pop(){
            self.receive(pac);
        }
    }
}
#[derive(Default)]
pub struct Network<'a,'c>{
    pub  nodes: [Node<'a,'a,'a>;N],
    pub  links: [[Link<'c>;N];N],
}
impl Network<'_,'_>{
    pub fn new(node_cap:[f64;N],link_cap:[[f64;N];N])->Self{
        let mut net = Network::default();
        let mut node_iter = net.nodes.iter_mut();
        for nc in node_cap{
            let  node:&mut Node<'_,'_,'_>= node_iter.next().unwrap();
            *node=Node::new(nc);
        }
        let mut link_line_iter = net.links.iter_mut();
        for lc_line in link_cap{
            let mut link_iter = link_line_iter.next().unwrap().iter_mut();
            for lc in lc_line{
                let  link :& mut Link<'_>= link_iter.next().unwrap();
                *link=Link::new(lc.clone());
            }
        }
        net
    }

}
impl <'a> Network<'a,'a>{
    pub fn process(&mut self,ana:&mut AnalyzeSystem,node_id: usize,mut pac:Packet<'a>){
        assert!(pac.is_live());
        pac.load *= pac.client.func[pac.stage()].scaling_factor;
        pac.add_stage();
        pac.pos=0;
        self.arrive_at(ana,node_id,pac);
    }
    //判断是否要接收（pair or unpair），还是转发（forward）
    pub fn arrive_at(&mut self,ana:&mut AnalyzeSystem,node_id: usize,pac: Packet<'a>){
        debug!(target:"net","arrive at {},{}",node_id,pac);
        //reach_end_flag, live_last_stage_flag
        match pac.status(){
            (true, true) =>{
                //leave network
                ana.analyze_packet_leave(pac);
            }
            (true, false) => {
                //process
                debug!(target:"net","let node {:<3} receive",node_id);
                self.nodes[node_id].receive_later(pac);
            }
            (false, _)=>{
                //forward
                debug!(target:"net","let link {:<3}->{:>3} receive\n",node_id,pac.next_pos());
                self.links[node_id][pac.next_pos()].receive_later(pac);
            }
        }
    }
    
    pub fn select_and_process(&mut self,ana:&mut AnalyzeSystem, node_id: usize){
        let mut cap = self.nodes[node_id].cap;
        debug!(target:"net","consuming top packets at {node_id} with cap = {cap}");
        if let Some(rem) = self.nodes[node_id].rem{
            if rem <= cap{
                cap-=rem;
                self.nodes[node_id].rem = None;
                let PacketPair(live_top,_) = self.nodes[node_id].paired_queue.pop().unwrap();
                    //process top packet
                debug!(target:"net","old big packet with rem {rem} consumed, remained cap is {cap}");
                self.process(ana,node_id,live_top);
            } else {
                self.nodes[node_id].rem=Some(rem-cap);
                debug!(target:"net","packet too big, remaing loads are:{}",self.nodes[node_id].rem.unwrap());
                return;
            }
        }
        while let Some(top) = self.nodes[node_id].paired_queue.peek(){
            
            if cap - 0f64 < 0.0000001{
                break;
            }
            let cost = top.cost();
            if cap < cost{
                self.nodes[node_id].rem = Some(cost-cap);
                debug!(target:"net","packet too big, remaing loads are:{}",self.nodes[node_id].rem.unwrap());
                
                return ;
            } else {
                cap -= cost;
                let PacketPair(live_top,_) = self.nodes[node_id].paired_queue.pop().unwrap();
                debug!(target:"net","consume top packet with client {} and load {cost}, cap are {}",live_top.client.client_id,cap);
                //process top packet;
                self.process(ana,node_id,live_top);
            }
        }
    }
    fn transmit(&mut self,ana:&mut AnalyzeSystem,j:usize,mut pac:Packet<'a>){
        pac.hop+=1;
        pac.pos+=1;
        self.arrive_at(ana,j, pac);
    }
    fn select_and_transmit(&mut self,ana:&mut AnalyzeSystem,i:usize,j:usize){
        let mut cap = self.links[i][j].cap;
        if self.links[i][j].rem != None{
            if self.links[i][j].rem.unwrap() <= cap{
                cap-=self.links[i][j].rem.unwrap();
                self.links[i][j].rem=None;
                let top = self.links[i][j].que.pop().unwrap();
                self.transmit(ana,j,top);
            } else {
                self.links[i][j].rem = Some(self.links[i][j].rem.unwrap()-cap);
                return ;
            }
        }
        while let Some(top) = self.links[i][j].que.peek(){
            if cap - 0f64 < 0.0000001{
                break;
            }
            let cost = top.load;
            if cap < cost{
                self.links[i][j].rem = Some(cost-cap);
                return ;
            } else {
                cap -= cost;
                let top = self.links[i][j].que.pop().unwrap();
                self.transmit(ana,j,top);
            }
        }
    }
    pub fn arrive(&mut self,ana:&mut AnalyzeSystem,pacs:(Packet<'a>,Vec<Packet<'a>>)){
        let (l_pac,s_pacs)=pacs;
        assert!(l_pac.is_live());
        ana.analyze_packet_arrive(&l_pac);
        
        self.arrive_at(ana,l_pac.now_pos(),l_pac);
        s_pacs.into_iter().for_each(|pac|{
            assert!(pac.is_live()==false);
            self.arrive_at(ana, pac.now_pos(), pac);
        });
    }
    pub fn tic_toc(&mut self,ana:&mut AnalyzeSystem){
        /*
            At each time slot t, for each node/link, give priority to the
        packets which have crossed the smallest number of edges in the
        ALG in the corresponding processing/transmission queue.
         */
        for i in 0..self.nodes.len(){
            self.select_and_process(ana,i);
        }
        for Edge(s,t,_) in edges{
            self.select_and_transmit(ana,s,t);
            self.select_and_transmit(ana,t,s);
        }
        for i in 0..self.nodes.len(){
            self.nodes[i].flush();
        }
        for Edge(s,t,_) in edges{
            self.links[s][t].flush();
            self.links[t][s].flush();
        }

        let mut in_link = [[0usize;N];N];
        let mut in_node_pair = [0usize;N];
        let mut in_node_unpair = [0usize;N];
        for Edge(s,t,_) in edges{
            in_link[s][t]=self.links[s][t].que.len();
            in_link[t][s]=self.links[t][s].que.len();
        }
        for i in 0..N{
            in_node_pair[i] = self.nodes[i].paired_queue.len();
            in_node_unpair[i] = self.nodes[i].unpaired_hash_map.len();
        }
        debug!(target:"net","{:?}\n{:?}\n{}",
            in_node_pair,
            in_node_unpair,
            Arr2::new("in link",in_link));
        
    }
}

impl fmt::Display for Client{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{},id:{},acc:{:?},func:{:?}",
        "client:"/*.to_string().truecolor(199,97,20)*/,
        self.client_id,
        self.acc,
        self.func)
    }
}
#[derive(new,Debug)]
pub struct Service{
    pub service_id: usize,
    pub process_cost: f64,
    pub merging_ratio: f64,
    pub database_id: usize,
    pub scaling_factor: f64,
}
/**
 * # 功能
 * 集成一系列服务`Service`，服务链式执行。保存了数据包进入网络的源节点与目标节点。
 * 
 */
#[derive(new,Debug)]
pub struct Client{
    pub client_id: usize,
    pub func: [Service;2],
    pub s: usize,
    pub t: usize,
    pub acc:[f64;3],
}
use crate::graph::{generate_paths,generate_single_path};
impl  Client{
    /**
     * # 功能
     * 根据floyd算法计算出的下一跳数组以及MIN-STAR算法处理出的一组处理节点来生成一组路径，包括一个live packet的路径和一组static packets的路径
     * 
     */
    pub fn generate_live_and_static_paths<const N:usize>
        (&self,p:&[[usize;N];N],mut nodes: Vec<usize>)
        ->(Vec<Vec<usize>>,Vec<Vec<usize>>){
        
        nodes.push(self.t);
        let static_paths:Vec<Vec<usize>> = zip(nodes.iter(),self.func.iter())
            .map(|(node,serv)|{
            generate_single_path(p,serv.database_id,*node)
        }).collect();
        assert_eq!(nodes.len(),static_paths.len()+1);
        (generate_paths(p,self.s,nodes),static_paths)
    }
}
