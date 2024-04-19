pub mod graph;
pub mod network;
pub mod virtualsystem;
pub mod matrix;
pub mod arr2;
pub mod analyzer;
pub mod corealgorithm;
pub mod packet;
pub mod database;
pub mod const_val;
pub mod range_step_to;
pub mod simulation;
use corealgorithm::RoutingAlgorithm;
use range_step_to::RangeStepInclusive as Range;
#[macro_use]
extern crate log;
extern crate log4rs;
use simulation::{simulate_alpha, simulate_average_delay, simulate_replica_index};
pub use const_val::*;
pub use simulation::SimulateResult;
use plotters::prelude::*;
struct _Configure{
    lambda: f64,
    rout_alg: RoutingAlgorithm,
    link_alpha: f64,//0~1
    node_alpha: f64,//0~1
    replica_index: usize,//1~N
}
pub const CIJ: f64=20f64;
pub const CI:[f64;N]=  [
                    5f64,5f64,5f64,5f64,
                    5f64,5f64,5f64,5f64,
                    10f64,10f64,10f64,10f64,
                    5f64,5f64,5f64,5f64
                    ];

fn average_delay_simulator(iter:impl Iterator<Item = f64>,rout:RoutingAlgorithm)
    ->Vec<(f64,f64)>{
    info!("{}:",rout);
    iter.scan(false,|state,lambda|{
        if *state == true{
            None
        }else{
            let res = simulate_average_delay(lambda,rout);
            if res.ave_delay>100.0{
                *state = true;
            }
            Some((lambda,res.ave_delay))
        }
    }).inspect(|(lambda,delay)|{
        info!("lambda: {:.3}, average delay: {:.4}",lambda,delay);
    }).collect::<Vec<_>>()
}
fn plot_ratio(data:Vec<(Vec<(f64,f64)>,RoutingAlgorithm,RGBColor)>){
    let root = BitMapBackend::new("Resource Occupation.png",(640,480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart= ChartBuilder::on(&root)
        .caption("Resource Occupation",("sans-serif",30))
        .set_label_area_size(LabelAreaPosition::Left,40)
        .set_label_area_size(LabelAreaPosition::Bottom,40)
        .build_cartesian_2d(0f64..1f64,0f64..1f64).unwrap();

    chart
        .configure_mesh()
        .x_desc("Processing resource occupation α1")
        .y_desc("Transmission resource occupation α2")
        .draw().unwrap();
    
    data.into_iter().for_each(|(ratio,alg,rgb)|{
        chart
            .draw_series(LineSeries::new(
                ratio,
                &rgb,
            )).unwrap()
            .label(alg.to_string())
            .legend(move|(x,y)|PathElement::new(vec![(x,y),(x+20,y)],rgb));
    });
    chart
        .configure_series_labels()
        .background_style(RGBColor(128,128,128))
        .draw().unwrap();
}
fn plot_average_delay(data:Vec<(Vec<(f64,f64)>,RoutingAlgorithm,RGBColor)>){
    let root = BitMapBackend::new("average_delay.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Network stability region", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0f64..14f64, 0f64..100f64).unwrap();

    chart
        .configure_mesh()
        .x_desc("Arrival rate [Mbps]")
        .y_desc("Average delay [slots]")
        .draw().unwrap();
    
    data.into_iter().for_each(|(delay,alg,col)|{
        chart
            .draw_series(LineSeries::new(
                delay,
            &col,
            )).unwrap()
            .label(alg.to_string())
            .legend(move|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], col));
    });
    chart
    .configure_series_labels()
    .background_style(RGBColor(128, 128, 128))
    .draw().unwrap();
}
fn map_link_ratio_with(rout_alg:RoutingAlgorithm)->impl Fn(f64)->(f64,f64){
    let epsilon = 0.001;
    move |node_ratio|->(f64,f64){
        let (mut l,mut r)=(0f64,1f64);
        let mut link_ratio:f64=(l+r)/2.0;
        while r-l>=epsilon{
            link_ratio = (l+r)/2.0;
            if simulate_alpha(node_ratio, link_ratio, rout_alg) == true{
                r=link_ratio;
            }else{
                l=link_ratio;
            }
        }
        (node_ratio,link_ratio)
    }
}
fn simulate_and_plot_average_delay(){
    // let di_dcnc_range = Range::new(5.0,2.0,5.0).chain(Range::new(90.01,0.01,13.0));
    // let s2l_range = Range::new(2.7,0.5,1.7).chain(Range::new(20.05,0.05,8.0));
    // let l2s_range = Range::new(9.6,0.5,9.5).chain(Range::new(20.05,0.05,7.0));
    let di_dcnc_range = Range::new(1.0,0.5,10.0).chain(Range::new(10.1,0.1,14.0));
    let s2l_range = Range::new(0.1,0.5,6.1).chain(Range::new(6.15,0.05,11.0));
    let l2s_range = Range::new(0.1,0.5,4.0).chain(Range::new(4.05,0.05,12.0));

    let di_dcnc_delay = average_delay_simulator(di_dcnc_range,RoutingAlgorithm::DI_DCNC);
    let s2l_delay = average_delay_simulator(s2l_range,RoutingAlgorithm::S2L);
    let l2s_delay = average_delay_simulator(l2s_range,RoutingAlgorithm::L2S);
    
    plot_average_delay(vec![
        (di_dcnc_delay,RoutingAlgorithm::DI_DCNC,BLUE),
        (s2l_delay,RoutingAlgorithm::S2L,RED),
        (l2s_delay,RoutingAlgorithm::L2S,GREEN),
    ]);
}
fn simulate_and_plot_ratio(){
    info!("\nDI-DCNC");
    let di_dcnc_ratio = 
        Range::new(0.01,0.01,0.3)
        .chain(Range::new(0.3,0.1,1.0))
        .map(map_link_ratio_with(RoutingAlgorithm::DI_DCNC))
        .inspect(|(node_ratio,link_ratio)| info!("ratio pair:({node_ratio},{link_ratio})"))
        .collect::<Vec<_>>();
    info!("\nS2L:");
    let s2l_ratio = 
        Range::new(0.01,0.04,0.17)
        .chain(Range::new(0.17,0.01,0.3))
        .chain(Range::new(0.3,0.1,1.0))
        .map(map_link_ratio_with(RoutingAlgorithm::S2L))
        .inspect(|(node_ratio,link_ratio)| info!("ratio pair:({node_ratio},{link_ratio})"))
        .collect::<Vec<_>>();

    info!("\nL2S:");
    let l2s_ratio = 
        Range::new(0.1,0.1,0.7)
        .chain(Range::new(0.71,0.01,1.0))
        .map(map_link_ratio_with(RoutingAlgorithm::L2S))
        .inspect(|(node_ratio,link_ratio)| info!("ratio pair:({node_ratio},{link_ratio})"))
        .collect::<Vec<_>>();
    
    
    plot_ratio(vec![
        (di_dcnc_ratio,RoutingAlgorithm::DI_DCNC,BLUE),
        (s2l_ratio,RoutingAlgorithm::S2L,RED),
        (l2s_ratio,RoutingAlgorithm::L2S,GREEN),
    ]);
}
fn main(){ 
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    simulate_and_plot_average_delay();
    simulate_and_plot_ratio();

}
