const SIZE: usize = 20;

fn get_matrix(row: usize, column: usize) -> Vec<Vec<bool>> {
    let mut matrix = Vec::new();

    for _ in 0..row {
        let mut new_row = Vec::new();
        for _ in 0..column {
            new_row.push(false);
        }
        matrix.push(new_row);
    }

    matrix
}

fn print_matrix(matrix: &Vec<Vec<bool>>) {
    println!("row(s): {}\ncolumn(s): {}\n", matrix.len(), matrix[0].len());

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] {
                print!("██");
            } else {
                print!("░░");
            }

        }
        println!();
    }
}

fn main() {
    let mut matrix = get_matrix(SIZE, SIZE);
    matrix[SIZE / 2][SIZE /2] = true;

    print_matrix(&matrix);
}

#[cfg(test)]
mod tests {
    use crate::get_matrix;
    const ROW_LENGTH: usize = 5;
    const COLUMN_LENGTH: usize = 3;

    #[test]
    fn test_matrix_row_length() {
        let matrix = get_matrix(ROW_LENGTH, COLUMN_LENGTH);
        assert_eq!(matrix.len(), ROW_LENGTH);
    }

    #[test]
    fn test_matrix_column_length() {
        let matrix = get_matrix(ROW_LENGTH, COLUMN_LENGTH);

        for index in 0..ROW_LENGTH {
            assert_eq!(matrix[index].len(), COLUMN_LENGTH);
        }
    }
}
