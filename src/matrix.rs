use std::iter::zip;

pub fn sub20<const N:usize>(arr1:&mut [[f64;N];N],b:f64){
    for line in arr1.iter_mut(){
        for a in line.iter_mut(){
            *a=if *a>b{*a-b} else{0f64};
            
        }
    }
}
pub fn sub11<const N:usize>(arr1:&mut[f64;N],arr2: &[f64;N]){
    let mut arr1_iter = arr1.iter_mut();
    for b in arr2{
        let a = arr1_iter.next().unwrap();
        *a= if *a>*b {*a-*b}else {0f64};
    }
}
/**
 * # 功能
 * 一维数组乘法，结果为二维数组
 * # 参数
 * 两个一维数组
 * # 返回值
 * 二维数组
 */
pub fn mul11_expand<const N:usize,const M:usize>
    (arr1 :&[f64;N],arr2:&[f64;M])->[[f64;N];M]{
    let mut res = [[0f64;N];M];
    let mut res_line_iter=res.iter_mut();
    for a in arr1{
        let mut res_ele_iter=res_line_iter.next().unwrap().iter_mut();
        for b in arr2{
            let  res_ele=res_ele_iter.next().unwrap();
            *res_ele=a*b;
        }
    }
    res
}
/**
 * # 功能
 * 二维数组（方阵）与一维数组相乘，结果为三维数组
 * 
 * # 参数
 * 二维数组（方阵）与一维数组
 * 
 * # 返回值
 * 三维数组
 */
pub fn mul21_expand<const N:usize, const M:usize>
    (arr1 :&[[f64;N];N],arr2: &[f64;M])->[[[f64;N];N];M]{
    let mut res = [[[0f64;N];N];M];
    let mut res_page_iter=res.iter_mut();
    for arr1_line in arr1{
        let mut res_line_iter=res_page_iter.next().unwrap().iter_mut();
        for arr1_ele in arr1_line{
            let mut res_ele_iter=res_line_iter.next().unwrap().iter_mut();
            for arr2_ele in arr2{
                let  res_ele=res_ele_iter.next().unwrap();
                *res_ele=arr1_ele*arr2_ele;
            }
        }
    }
    res
}
pub fn div11_each_gen<const N:usize>
    (arr1:&[f64;N],arr2:&[f64;N])->[f64;N]{
    let mut res:[f64;N]=[0f64;N];
    let mut arr2_iter=arr2.iter();
    let mut res_iter=res.iter_mut();
    for a in arr1{
        let b = arr2_iter.next().unwrap();
        let c = res_iter.next().unwrap();
        if *b!=0f64 {
            *c=a/ *b;
        }
    }
    res
}
pub fn add11_each<const N:usize>
    (arr1:&mut [f64;N],arr2:&[f64;N]){
    let mut arr2_iter=arr2.iter();
    for a in arr1.iter_mut(){
        let b = arr2_iter.next().unwrap();
        if *b!=0f64 {
            *a+= *b;
        }
    }
}
pub fn div10_gen<const N:usize>
    (arr1:&[f64;N],b: &f64)->[f64;N]{
    let mut res:[f64;N]=[0f64;N];
    let mut res_iter=res.iter_mut();
    for a in arr1{
        let c = res_iter.next().unwrap();
        if *b!=0f64 {
            *c=a/ b;
        }
    }
    res
}
pub fn div20_gen<const N:usize>(arr1:&[[f64;N];N],b: &f64)->[[f64;N];N]{
    let mut res:[[f64;N];N]=[[0f64;N];N];
    let mut res_line_iter=res.iter_mut();
    for line in arr1{
        let mut res_ele_iter = res_line_iter.next().unwrap().iter_mut();
        for a in line{
            let c = res_ele_iter.next().unwrap();
            if *b!= 0f64{
                *c = a/ *b;
            }
        }
    }
    res
}
pub fn div20<const N:usize>(arr1:&mut [[f64;N];N],b: f64){
    for line in arr1{
        for a in line{
            if b!= 0f64{
                *a /= b;
            }
        }
    }
}
pub fn add22_each<const N:usize>(arr1:&mut [[f64;N];N],arr2:& [[f64;N];N]){
    let mut arr1_line_iter = arr1.iter_mut();
    for arr2_line in arr2{
        let mut arr1_ele_iter = arr1_line_iter.next().unwrap().iter_mut();
        for b in arr2_line{
            let a = arr1_ele_iter.next().unwrap();
            *a +=*b;
        }
    }
}
pub fn mul10_gen<const N:usize>(arr1:&[f64;N],b:f64)->[f64;N]{
    let mut ans = [0f64;N];
    zip(ans.iter_mut(),arr1.iter()).for_each(|(c,a)|{
        *c=*a*b;
    });
    ans
}
#[cfg(test)]
mod test{
    use crate::matrix::*;
    use approx::relative_eq;
    #[test]
    fn test_sub11(){
        let mut a=[1f64,3f64];
        sub11(&mut a,&[2f64,2f64]);
        assert_eq!(a,[0f64,1f64]);
    }
    #[test]
    fn test_sub20(){
        let mut a=[[1f64,2f64],[3f64,4f64]];
        sub20(&mut a,2f64);
        assert_eq!(a,[[0f64,0f64],[1f64,2f64]]);
    }
    #[test]
    fn test_mul11_expand(){
        let a=[0f64,1f64,0.2f64];
        let b=[3f64,0.8f64,4f64];
        let res=mul11_expand(&a,&b);
        let ans=[[0f64,0f64,0f64],
                [3f64,0.8f64,4f64],
                [0.6f64,0.16f64,0.8f64]];
        relative_eq22(res,ans);
    }
    #[test]
    fn test_mul21_expand(){
        let a=[[1f64,2f64],[3f64,4f64]];
        let b=[2f64,3f64];
        let res=mul21_expand(&a,&b);
        let ans=[[[2f64,4f64],[6f64,8f64]],[[3f64,6f64],[9f64,12f64]]];
        relative_eq33(ans,res);
    }
    #[test]
    fn test_add22_each(){
        let mut a=[[1f64,2f64],[3f64,4f64]];
        let b=[[1f64,2f64],[3f64,4f64]];
        add22_each(&mut a,&b);
        let ans=[[2f64,4f64],[6f64,8f64]];
        relative_eq22(ans,a);
    }
    #[test]
    fn test_div11_each_gen(){
        let a = [1f64,2f64];
        let b = [3f64,9f64];
        let res = div11_each_gen(&a,&b);
        relative_eq11(res,[1f64/3f64,2f64/9f64]);
    }
    #[test]
    fn test_div10_gen(){
        let a = [2f64,9f64];
        relative_eq11(div10_gen(&a,&4f64),[0.5f64,2.25f64]);
    }
    #[test]
    fn test_div20_gen(){

        let a = [[2f64,9f64],[4f64,f64::INFINITY]];
        let b = 4f64;
        relative_eq22(div20_gen(&a,&b),[[0.5f64,2.25f64],[1f64,f64::INFINITY]]);
    }
    fn relative_eq11<const N:usize>
        (arr1:[f64;N],arr2:[f64;N]){
            let mut arr2_ele_iter = arr2.into_iter();
            for left in arr1{
                let right = arr2_ele_iter.next().unwrap();
                let _ = relative_eq!(left,right);
            }
        }
    fn relative_eq22<const N:usize>
        (arr1:[[f64;N];N],arr2:[[f64;N];N]){
        let mut arr2_line_iter=arr2.into_iter();
        for arr1_line in arr1{
            let mut arr2_ele_iter = arr2_line_iter.next().unwrap().into_iter();
            for left in arr1_line{
                let right = arr2_ele_iter.next().unwrap();
                let _ = relative_eq!(left,right);
            }
        }
    }
    fn relative_eq33<const N:usize,const M:usize>
        (arr1:[[[f64;N];N];M],arr2:[[[f64;N];N];M]){
        let mut arr2_page_iter=arr2.into_iter();
        for arr1_page in arr1{
            let mut arr2_line_iter = arr2_page_iter.next().unwrap().into_iter();
            for arr1_line in arr1_page{
                let mut arr2_ele_iter = arr2_line_iter.next().unwrap().into_iter();
                for left in arr1_line{
                    let right = arr2_ele_iter.next().unwrap();
                    let _ = relative_eq!(left,right);
                }
            }
        }
    }
}