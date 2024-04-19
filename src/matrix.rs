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