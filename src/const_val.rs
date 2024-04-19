pub const CIJ: f64=20f64;
pub const CI:[f64;N]=  [
                    5f64,5f64,5f64,5f64,
                    5f64,5f64,5f64,5f64,
                    10f64,10f64,10f64,10f64,
                    5f64,5f64,5f64,5f64
                    ];

macro_rules! i{
    ($c:literal) => {
        (($c as u8)+8-('A' as u8)) as usize
    };
}

pub const TIME_LIMIT :usize= 10000;
pub const N :usize= 16;
pub const M :usize= 8;

pub struct Edge(pub usize,pub usize,pub f64);
#[allow(non_upper_case_globals)]
pub const edges:[Edge;24]= [
    Edge(i!('E'),3-1,1f64),
    Edge(i!('E'),1-1,1f64),
    Edge(i!('F'),2-1,1f64),
    Edge(i!('F'),4-1,1f64),
    Edge(i!('H'),6-1,1f64),
    Edge(i!('H'),8-1,1f64),
    Edge(i!('G'),7-1,1f64),
    Edge(i!('G'),5-1,1f64),
    Edge(i!('A'),1-1,1f64),
    Edge(i!('A'),3-1,1f64),
    Edge(i!('B'),2-1,1f64),
    Edge(i!('B'),4-1,1f64),
    Edge(i!('C'),5-1,1f64),
    Edge(i!('C'),7-1,1f64),
    Edge(i!('D'),6-1,1f64),
    Edge(i!('D'),8-1,1f64),
    Edge(i!('A'),i!('B'),1f64),
    Edge(i!('A'),i!('C'),1f64),
    Edge(i!('D'),i!('C'),1f64),
    Edge(i!('D'),i!('B'),1f64),
    Edge(1-1,2-1,1f64),
    Edge(3-1,5-1,1f64),
    Edge(4-1,6-1,1f64),
    Edge(7-1,8-1,1f64),
];
#[derive(Debug)]
pub struct Service(f64,f64,usize,f64);
impl Service{
    pub const fn process_cost(&self)->f64{
        self.0
    }
    pub const fn merging_ratio(&self)->f64{
        self.1
    }
    pub const fn db_ind(&self)->usize{
        self.2
    }
    pub const fn scaling_factor(&self)->f64{
        self.3
    }
}
pub const  service0:Service = Service(0.2f64,1f64,1-1,1f64);
pub const  service1:Service = Service(0.2f64,1f64,2-1,2f64);
pub const  service2:Service = Service(0.5f64,2f64,3-1,1f64);
pub const  service3:Service = Service(0.5f64,3f64,4-1,0.5f64);
pub const  service4:Service = Service(0.1f64,1f64,5-1,1f64);
pub const  service5:Service = Service(0.1f64,1f64,6-1,3f64);
pub const  service6:Service = Service(1f64,5f64,7-1,0.5f64);
pub const  service7:Service = Service(1f64,10f64,8-1,1f64/3f64);



