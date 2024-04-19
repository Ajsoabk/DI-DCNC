#[derive(Clone)]
pub struct RangeStepInclusive{
    status: f64,
    step:f64,
    right_end:f64
}
impl RangeStepInclusive{
    pub fn new(status:f64,step:f64,right_end:f64)-> Self{
        Self{
            status,
            step,
            right_end,
        }
    }
}
impl Iterator for RangeStepInclusive{
    type Item = f64;
    fn next(&mut self)-> Option<Self::Item>{
        let ret = self.status;
        self.status+=self.step;
        if ret<=self.right_end{
            Some(ret)
        }
        else{
            None
        }
    } 
}
#[test]
fn test_iter(){
    let iter = RangeStepInclusive::new(1.0,2.0,5.0);
    assert_eq!(iter.collect::<Vec<_>>(),vec![1.0,3.0,5.0]);

    let iter1=RangeStepInclusive::new(1.0,2.0,9.0);
    let iter2=RangeStepInclusive::new(9.0,0.3,10.0);
    assert_eq!(iter1.chain(iter2).collect::<Vec<_>>(),vec![1.0,3.0,5.0,7.0,9.0,9.0,9.3,9.6,9.9]);
}