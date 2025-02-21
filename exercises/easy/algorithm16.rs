/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

fn rotate_element(
    matrix: &mut Vec<Vec<i32>>,
    set: &mut HashSet<(usize, usize)>,
    a: usize,
    b: usize,
    val: i32,
) {
    // (a, b) 为转移起始点
    let height_o: usize = matrix.len();
    let width_o: usize = matrix[0].len();
    let dest_a: usize = b.clone();
    let dest_b: usize = height_o - a - 1;
    let mut temp: i32;
    if set.contains(&(a, b)) {
        return;
    } else {
        set.insert((a, b));
    }
    if dest_a >= height_o || dest_b >= width_o {
        return;
    } else {
        //
        temp = matrix[dest_a][dest_b];
        matrix[dest_a][dest_b] = val;
        rotate_element(matrix, set, dest_a, dest_b, temp);
    }
}

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place
    //let mut temp: i32;
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let height_o: usize = matrix.len();
    let width_o: usize = matrix[0].len();

    if height_o > width_o {
        // 先补充目标矩阵右侧
        for i in 0..width_o {
            // 按行填充
            for j in 0..height_o - width_o {
                let temp_v: i32 = matrix[height_o - width_o - j - 1][i];
                //let temp_v: i32 = (i + 1) as i32;
                matrix[i].push(temp_v.clone());
                set.insert((height_o - width_o - j - 1, i));
            }
        }

        // 修改重叠部分
        for i in height_o - width_o..height_o {
            for j in 0..width_o {
                rotate_element(matrix, &mut set, i, j, matrix[i][j]);
            }
        }

        // 删除多余元素
        for i in 0..height_o - width_o {
            matrix.pop();
        }
        return;
    } else if height_o < width_o {
        // 先补充目标矩阵下侧
        let e = height_o - width_o;
        for i in 0..width_o - height_o - 1 {
            // 按行填充
            let mut temp_vec: Vec<i32> = Vec::new();
            for j in 0..height_o {
                temp_vec.push(matrix[height_o - j - 1][i].clone());
                set.insert((height_o - j - 1, i));
            }
            matrix.push(temp_vec);
        }
        // 修改重叠部分
        for i in 0..height_o {
            for j in 0..height_o {
                rotate_element(matrix, &mut set, i, j, matrix[i][j]);
            }
        }
        for i in 0..height_o {
            for j in 0..height_o {
                matrix[i].pop();
            }
        }
        return;
    } else {
        for i in 0..height_o {
            for j in 0..width_o {
                rotate_element(matrix, &mut set, i, j, matrix[i][j]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
    }
}
