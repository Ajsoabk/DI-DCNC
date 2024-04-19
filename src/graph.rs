/**
# 功能
用floyd算法计算出任意点之间的最短路，并提供任意两点间最短路的下一个节点

# 参数
cost，f64类型的N*N二维数组，表示任意两点间代价，如果两点间没有边，代价应当为+Inf_f64

# 返回值
f64类型的N*N二维数组，表示任意两点间最短路的代价
usize类型的N*N二维数组，表示任意两点间最短路的下一个节点

# Example1
```
use di_dcnc_rs::graph::{Edge,EdgeType,create_graph,floyd};

const edge_cnt:usize=4;
const node_cnt:usize=4;
let edge_array=[Edge(1,2,1f64),Edge(1,3,5f64),Edge(2,3,2f64),Edge(3,4,3f64),];

let bgw = create_graph::<node_cnt,edge_cnt>(&edge_array,EdgeType::BidirectionalWeighted);
let (bgw,p) = floyd(bgw);
const inf:f64 = f64::INFINITY;
assert_eq!(bgw,[[0f64,1f64,3f64,6f64],
                [1f64,0f64,2f64,5f64],
                [3f64,2f64,0f64,3f64],
                [6f64,5f64,3f64,0f64],]);
assert_eq!(p,  [[1,2,2,2],
                [1,2,3,3],
                [2,2,3,4],
                [3,3,3,4],])


```
*/
use crate::N;
use rand::prelude::*;

pub fn floyd
    (mut d :[[f64;N];N])-> ([[f64;N];N],[[usize;N];N],[[usize;N];N]){
    let mut p = [[N+1usize;N];N];
    let mut rng = rand::thread_rng();
    let mut delta:[[f64;N];N]=[[f64::INFINITY;N];N];
    let mut hop_cnt = [[300usize;N];N];
    for i in 0..N{
        for j in 0..N{
            if d[i][j]!=f64::INFINITY || i==j{
                p[i][j]=j;
            }
            if i==j{
                delta[i][j]=0f64;
                hop_cnt[i][j]=0usize;
            }
            else if d[i][j]!=f64::INFINITY{
                hop_cnt[i][j]=1usize;
                delta[i][j]=rng.gen::<f64>()/1000.0+0.0001;
            }
        }
    }
    for k in 0..N{
        for i in 0..N{
            for j in 0..N{
                let mut flag=false;
                if d[i][k]+d[k][j]<d[i][j]{
                    flag=true;
                }else if d[i][k]+d[k][j]==d[i][j]{
                    if hop_cnt[i][k]+hop_cnt[k][j]<hop_cnt[i][j]{
                        flag=true;
                    }else if hop_cnt[i][k]+hop_cnt[k][j]==hop_cnt[i][j]{
                        if delta[i][k]+delta[k][j]<delta[i][j]{
                            flag=true;
                        }
                    }
                }
                if flag==true 
                {
                    d[i][j]=d[i][k]+d[k][j];
                    delta[i][j]=delta[i][k]+delta[k][j];
                    hop_cnt[i][j]=hop_cnt[i][k]+hop_cnt[k][j];
                    p[i][j]=p[i][k];
                }
            }
        }
    }
    (d,p,hop_cnt)
}

/**
 * # 功能
 * 根据由源节点、处理节点与目的节点组成的向量生成一组路径
 * 
 * # 输入
 * 一个表示任意两点间最短路的下一节点的二维数组。
 * 一个由源节点、处理节点与目的节点组成的向量
 * 
 * # 返回值
 * 一组路径，每个路径都是一个Vec
 * 
 * # Example1
 * ```
 * use di_dcnc_rs::graph::{Edge,EdgeType,create_graph,floyd,generate_paths};
 * use std::collections::VecDeque;
 * const edge_cnt:usize=4;
 * const node_cnt:usize=4;
 * let edge_array=[Edge(1,2,1f64),Edge(1,3,5f64),Edge(2,3,2f64),Edge(3,4,3f64),];
 * 
 * let bgw = create_graph::<node_cnt,edge_cnt>(&edge_array,EdgeType::BidirectionalWeighted);
 * let (_,p) = floyd(bgw);
 * let node_list=VecDeque::from([1,4,2]);
 * let paths=generate_paths(&p,node_list);
 * let ans=vec![vec![1,2,3,4],vec![4,3,2]];
 * assert_eq!(paths,ans);
 * 
 * ```
 */
use std::collections::VecDeque;
pub fn generate_paths<const N:usize>
    (p :&[[usize;N];N],mut node: VecDeque<usize>)->Vec<Vec<usize>>{
    let mut result:Vec<Vec<usize>>=Vec::with_capacity(node.len());
    let mut s = node.pop_front().unwrap();
    for t in node{
        result.push(generate_single_path(p,s,t));
        s=t;
    }
    result
}
pub fn generate_single_path<const N:usize>
    (p :&[[usize;N];N],mut s:usize,t:usize)->Vec<usize>{
    let mut vec = Vec::with_capacity(N);
    // print!("from {} to {} :",s.to_string(),t.to_string());
    loop{
        vec.push(s);
        if s==t {
            // println!("{:?}",vec);
            return vec;
        }
        if vec.len()>N{
            panic!("looping while generating single path");
        }
        s=p[s][t];
    }
}