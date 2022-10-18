const SIZE: usize = 5;

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

fn main() {
    let matrix = get_matrix(SIZE, SIZE);
    println!("{}", matrix.len());

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
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
