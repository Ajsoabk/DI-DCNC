use derive_new::new;
#[derive(new)]
pub struct Arr2<T,const N:usize>{
    name :&'static str,
    arr:[[T;N];N]
}

use std::fmt;
#[allow(non_upper_case_globals)]
impl <const N:usize>fmt::Display for Arr2<f64,N>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        const pad :usize = 9;
        write!(f,"{:<1$}",self.name/*.truecolor(199,97,20)*/,pad)?;
        for i in 0..N{
            write!(f,"{:<1$}",i/* .to_string().truecolor(199,97,20)*/,pad)?;
        }
        write!(f,"\n")?;
        for (i,line) in self.arr.iter().enumerate(){
            write!(f,"{:<1$}",i/* .to_string().truecolor(199,97,20)*/,pad)?;
            for ele in line{
                write!(f,"{:<1$.3}",ele,pad)?;
            }
            write!(f,"\n")?;
        }
        write!(f,"\n")
    }
} 
#[allow(non_upper_case_globals)]
impl <const N:usize>fmt::Display for Arr2<usize,N>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        const pad :usize = 7;
        write!(f,"{:<1$}",self.name/*.truecolor(199,97,20)*/,pad)?;
        for i in 0..N{
            write!(f,"{:<1$}",i/* .to_string().truecolor(199,97,20)*/,pad)?;
        }
        write!(f,"\n")?;
        for (i,line) in self.arr.iter().enumerate(){
            write!(f,"{:<1$}",i/* .to_string().truecolor(199,97,20)*/,pad)?;
            for ele in line{
                write!(f,"{:<1$}",ele,pad)?;
            }
            write!(f,"\n")?;
        }
        write!(f,"\n")
    }
} 
// #[test]
// fn test_arr2(){
//     let a = [[1.0,2.0],[3.0,4.0]];
//     eprintln!("{}",Arr2::new("a".to_string(),a));
// }