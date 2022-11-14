use std::fs::{write, read_to_string,File};
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use serde_json;

pub type OriginalVec = Vec<Vec<usize>>;
pub type SparseVecType = OriginalVec;

#[derive(Serialize, Deserialize, Debug)]
pub struct SparseVec {
    pub sparse_vec: SparseVecType,
}
#[allow(dead_code)]
pub fn origin_to_sparse(origin: &OriginalVec) -> SparseVec {
    // 1. 先遍历二维数组 得到非 0 数据的个数
    let mut sum: usize = 0;
    let s: usize = 0;
    for arr in origin {
        for value in arr {
            if value != &s {
                sum += 1;
            }
        }
    }

    // 2. 创建对应的稀疏数组，这里用向量包裹起来
    let mut sparse_vec: SparseVecType = vec![vec![origin.len(), origin[0].len(), sum]];

    // 3. 遍历二维数组，将非 0 的值存放到 sparse_vec 中
    let mut i: usize = 0;
    while i < origin.len() - 1 {
        i = i + 1;
        let mut j = 0;
        while j < origin[i].len() - 1 {
            j = j + 1;
            if origin[i][j] != 0 {
                sparse_vec.push(vec![i, j, origin[i][j]]);
            }
        }
    }
    SparseVec{ sparse_vec }
}
#[allow(dead_code)]
pub fn sparse_to_origin(sparse: &SparseVec) -> OriginalVec {
    let sparse_vec=&sparse.sparse_vec;
    // 先读取稀疏数组的第一行，创建出符合长度要求的原类型数组
    let first = &sparse_vec[0];
    let mut original_vec: OriginalVec = vec![vec![0; first[1]]; first[0]];

    // 再读取剩下的行数，将非零元素重新赋值
    let mut i: usize = 1;
    while i <= sparse_vec[0][2] {
        original_vec[sparse_vec[i][0]][sparse_vec[i][1]] = sparse_vec[i][2];
        i += 1;
    }
    original_vec
}

/**
 * @desc 接收稀疏数组和json文件名，经过处理，把稀疏数组存为json文件
 */
#[allow(dead_code)]
pub fn write_to_json_file(sparse: &SparseVec,filename:&str) {
    //    let output_path = std::env::args().nth(2).unwrap();

    // 数组转成json字符串
    let json_str_sparse = serde_json::to_string_pretty(sparse)
        .expect("转化为 json 字符串失败！");

    if Path::new(filename).exists() == true {
        write(
            filename,
            &json_str_sparse
        )
            .expect("写入 json 文件失败！");
    } else {
        match File::create(filename) {
            Ok(_) => {
                write(
                    filename,
                    &json_str_sparse
                )
                    .expect("写入新建文件失败！");
            }
            Err(_) => {
                println!("新建文件失败！");
            }
        };
    }
}
#[allow(dead_code)]
pub fn read_from_json_file(filename:&str)->SparseVec{
    let sparse_vec = {
        let json_str = read_to_string(filename).expect("读取文件失败！");
        // 从json字符串中加载稀疏数组的rust数据结构.
        serde_json::from_str::<SparseVec>(&json_str).expect("转化成rust数据失败！")
    };
    sparse_vec
}