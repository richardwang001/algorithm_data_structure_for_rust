mod lib;
use sparse_vec::*;

fn main() {
    // 创建一个原始的二维数组 11 * 11
    // 0: 表示没有棋子， 1 表示 黑子 2 表示蓝子
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_to_sparse() {
        let filename = "sparse_json.json";

        let mut chess_vec: OriginalVec = vec![vec![0; 11]; 11];
        chess_vec[1][2] = 1;
        chess_vec[2][3] = 2;
        chess_vec[4][5] = 2;

        let parsed = origin_to_sparse(&chess_vec);
        write_to_json_file(&parsed,filename);

        assert_eq!(parsed.sparse_vec[0][2],3);
        assert_eq!(parsed.sparse_vec[2][1],3);
    }

    #[test]
    fn test_sparse_to_origin(){
        let filename = "sparse_json.json";
        let parsed = read_from_json_file(filename);
        let origin = sparse_to_origin(&parsed);
        for a in &origin{
            println!("{:?}", a);
        };
        assert_eq!(origin[1][2],1);
        assert_eq!(origin[2][3],2);
        assert_eq!(origin[4][5],2);
        assert_eq!(origin.len(),11);
    }
}
