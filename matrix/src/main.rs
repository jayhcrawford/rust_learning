fn produce_matrix(matrix: &mut Vec<Vec<i32>>, size_m: i32, size_n: i32) {
    for _ in 0..size_m {
        let mut row: Vec<i32> = Vec::new();
        for idx in 0..(size_n + 1) {
            row.push(idx);
        }
        matrix.push(row);
    }
}

fn print_matrix_row(matrix_row: Vec<i32>) {
    let output = matrix_row
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    print!("{output}");
}

fn main() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let m: i32 = 10;
    let n: i32 = 10;

    produce_matrix(&mut matrix, m, n);

    let mut idx: i32 = 0;
    for row in matrix {
        print!(r"[");
        print_matrix_row(row);
        print!(r"]");
        idx += 1;
        if idx < n {
            println!(",");
        } else {
            println!();
        }
    }
}
