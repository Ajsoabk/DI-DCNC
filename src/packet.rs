

use std::cmp::Ordering;
use crate::network::Client;
/**
 * # 功能
 * 存储在网络中移动的包的信息，包括跳数、到达网络的时间、所属用户、在服务链中所处的阶段以及每个阶段的路径
 * 
 * # Example1
 * use di_dcnc_rs::graph::{Edge,EdgeType,create_graph,floyd,generate_paths};
 * use di_dcnc_rs::network::Packet;
 * const edge_cnt:usize=4;
 * const node_cnt:usize=4;
 * let edge_array=[Edge(1,2,1f64),Edge(1,3,5f64),Edge(2,3,2f64),Edge(3,4,3f64f64),];
 * 
 * let bgw = create_graph::<node_cnt,edge_cnt>(&edge_array,EdgeType::BidirectionalWeighted);
 * let (_,p) = floyd(bgw);
 * let node_list=vec![1,4,2];
 * let paths=generate_paths(p,node_list);
 * let ans=vec![vec![1,2,3,4],vec![4,3,2]];
 */
#[derive(Debug)]
pub struct Packet<'client>{
    pub pac_id: usize,
    pub hop : usize,
    pub cnt : f64,
    pub load : f64,
    pub generate_time : usize,
    pub client: &'client Client,
    stage: usize,
    pub pos: usize,
    static_stage: usize,
    pub paths: Vec<Vec<usize>>
}
impl <'a> Packet<'a>{
    pub fn new(pac_id:usize,pac_load:f64,t:usize,c: &'a Client,paths:Vec<Vec<usize>>)->Self{
        Self{
            pac_id,
            cnt:pac_load,
            load:pac_load,
            generate_time:t,
            client:c,
            hop: 0usize,
            stage: 0usize,
            pos: 0usize,
            static_stage: 0usize,
            paths: paths,
        }
    }
}
use std::fmt;
impl fmt::Display for Packet<'_>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{},hop:{},load:{:.3},{}:{},client id:{},stage:{},pos:{}, path:{:?}",
        "packet :",
        self.hop,
        self.load,
        "t",
        self.generate_time,
        self.client.client_id,
        self.stage(),
        self.pos,
        self.paths,)
    }
}

impl Packet<'_>{

    pub fn set_static_stage(&mut self,x:usize){
        
        #[cfg(feature="check_variable")]
        assert!(self.is_live()==false);
        self.static_stage=x;
    }
    pub fn add_stage(&mut self){
        #[cfg(feature = "check_variable")]
        assert!(self.is_live());
        self.stage+=1;
    }
    pub fn stage(&self) -> usize{
        if self.is_live(){
            self.stage
        }else{
            self.static_stage
        }
    }
    pub fn is_live(&self) -> bool{
        self.paths.len()!=1
    }
    //是否需要转发，是否需要处理
    //reach_end_flag, live_last_stage_flag
    pub fn status(&self) ->(bool,bool){
        (
            self.pos + 1 == self.paths[self.stage].len(),
            self.stage + 1 == self.paths.len() && self.paths.len()!=1,
        )
    }
    pub fn now_pos(&self) -> usize{
        self.paths[self.stage][self.pos]
    } 
    pub fn next_pos(&self) -> usize{
        self.paths[self.stage][self.pos+1]
    }
}
impl Ord for Packet<'_>{
    fn cmp(&self, other :&Self) -> Ordering{
        other.hop.cmp(&self.hop)
    }
}
impl PartialOrd for Packet<'_>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(other))

    }
}
impl PartialEq for Packet<'_>{
    fn eq(&self,other: &Self) ->bool{
        self.hop == other.hop
    }
}
impl Eq for Packet<'_>{}
pub struct PacketPair<'a>(pub Packet<'a>,pub Packet<'a>);
impl Ord for PacketPair<'_>{
    fn cmp(&self, other: &Self)->Ordering{
        (other.0.hop).cmp(&(self.0.hop))
    }
}
impl PacketPair<'_>{
    pub fn cost(&self)->f64{
        self.0.load*self.0.client.func[self.0.stage].process_cost
    }
}

impl PartialOrd for PacketPair<'_>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketPair<'_>{
    fn eq(&self, other:&Self) -> bool{
        self.0.hop == other.0.hop
    }
}
impl Eq for PacketPair<'_>{}
