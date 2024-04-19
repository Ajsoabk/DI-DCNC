
use crate::packet::Packet;
#[derive(Default)]
pub struct AnalyzeSystem{
    pub t: usize,
    arrive_cnt: f64,
    leave_cnt: f64,
    finished_delay: f64,
    in_network_delay: f64,
    in_network_count: f64,
}
impl AnalyzeSystem{
    pub fn total_delay(&self)-> f64{
        self.finished_delay+self.in_network_delay
    }
    pub fn total_count(&self)-> f64{
        self.arrive_cnt
    }
    pub fn analyze_packet_arrive(&mut self, pac: &Packet){
        debug!(target: "ana","client {} arrive, load:{:7.3}, paths:{:?}"
        ,pac.client.client_id,pac.cnt,pac.paths);
        self.arrive_cnt+=pac.cnt;
        self.in_network_count+=pac.cnt;
    }
    pub fn analyze_packet_leave(&mut self, pac:Packet){
        #[cfg(feature="check_variable")]
        assert!(pac.is_live());
        self.leave_cnt+=pac.cnt;
        self.in_network_count-=pac.cnt;
        self.finished_delay+=(self.t-pac.generate_time) as f64*pac.cnt;
        self.in_network_delay-=(self.t-pac.generate_time) as f64*pac.cnt;
        
        #[cfg(feature="check_variable")]
        assert!(self.leave_cnt<=self.arrive_cnt);
        debug!(target: "ana","client {} packet t:{} leaving network with paths:{:?}"
        ,pac.client.client_id,pac.generate_time,pac.paths);
    }
    pub fn tic_toc(&mut self){
        self.t+=1;
        self.in_network_delay+=self.in_network_count;
    }
    pub fn average_delay(&self) -> f64{
        self.total_delay() / self.total_count() 
    }
    pub fn report(&self)->String{
        format!("{:<6}time slots simulated, {:<4} packets still in network, arrives {:<7}, leaves {:<7}, tot_delay {:<9}, average delay {:.4}",
        self.t,self.in_network_count,self.arrive_cnt,self.leave_cnt,self.total_delay(),self.average_delay())
    }
}