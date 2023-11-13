fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];

    for (i, el) in matrix.iter().enumerate() {
        for (j, _) in el.iter().enumerate() {
            new_matrix[j][i] = matrix[i][j];
        }
    }

    return new_matrix;
}

fn pretty_print(matrix: [[i32; 3]; 3]) {
    for i in matrix {
        println!("{i:?}");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(matrix);

    println!();

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(transposed);
}
