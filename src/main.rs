const SIZE: usize = 20;

struct Game {
    generation: i32,
    grid: Vec<Vec<bool>>
}

impl Game {
    fn new(row: usize, column: usize) -> Self {
        let mut game = Vec::new();
    
        for _ in 0..row {
            let mut new_row = Vec::new();
            for _ in 0..column {
                new_row.push(false);
            }
            game.push(new_row);
        }
        
        Game {
            generation: 0,
            grid: game
        }
    }

    fn show(&self) {
        println!(
            "generation: {}\nrow(s): {}\ncolumn(s): {}\n",
            self.generation, self.grid.len(), self.grid[0].len()
        );
    
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] {
                    print!("██");
                } else {
                    print!("░░");
                }
    
            }
            println!();
        }
    }

    fn evolve(&mut self) {
        unimplemented!()
    }
    
    fn rules(&self, row: usize, column: usize) -> bool {
        match self.count_neighbors(row, column) {
            2 | 3 => true,
            _ => false
        }
    }

    fn count_neighbors(&self, row: usize, column: usize) -> usize {
        let mut count = 0;

        let a;
        if row == 0 {
            a = 1;
        } else {
            a = 0;
        }

        let b;
        if row == (self.grid.len() - 1) {
            b = 1;
        } else {
            b = 2;
        }

        let c;
        if column == 0 {
            c = 1;
        } else {
            c = 0;
        }

        let d;
        if column == (self.grid[0].len() - 1) {
            d = 1;
        } else {
            d = 2;
        }

        for i in a..=b {
            for j in c..=d {
                if self.grid[row + i - 1][column + j - 1] {
                    count += 1;
                }
            }
        }

        if self.grid[row][column] {
            count -= 1;
        }
        
        dbg!(count);

        count
    }

}


fn main() {
    let mut game = Game::new(SIZE, SIZE);
    game.grid[SIZE / 2][SIZE /2] = true;

    let cell = (SIZE / 2, SIZE - 1);

    game.grid[cell.0 - 1][cell.1 - 1] = true;
    game.grid[cell.0][cell.1 - 1] = true;
    game.grid[cell.0 + 1][cell.1 - 1] = true;
    game.grid[cell.0 - 1][cell.1] = true;
    game.grid[cell.0 + 1][cell.1] = true;

    println!("({}, {}) => {}", cell.0, cell.1, game.count_neighbors(cell.0, cell.1));

    game.show();
}

#[cfg(test)]
mod tests {
    use crate::Game;
    const ROW_LENGTH: usize = 5;
    const COLUMN_LENGTH: usize = 3;

    #[test]
    fn test_grid_row_length() {
        let game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        assert_eq!(game.grid.len(), ROW_LENGTH);
    }

    #[test]
    fn test_grid_column_length() {
        let game = Game::new(ROW_LENGTH, COLUMN_LENGTH);

        for index in 0..ROW_LENGTH {
            assert_eq!(game.grid[index].len(), COLUMN_LENGTH);
        }
    }
    
    #[test]
    fn test_count_neighbors_no_neigtbors() {
        let game = Game::new(ROW_LENGTH, COLUMN_LENGTH);

        assert_eq!(game.count_neighbors(ROW_LENGTH / 2, COLUMN_LENGTH / 2), 0);
    }

    #[test]
    fn test_count_neighbors_one_neigtbor_center() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let center = ((ROW_LENGTH / 2), (COLUMN_LENGTH / 2));

        game.grid[center.0][center.1 - 1] = true;

        assert_eq!(game.count_neighbors(center.0, center.1), 1);
    }

    #[test]
    fn test_count_neighbors_multiple_neigtbors_center() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let center = ((ROW_LENGTH / 2), (COLUMN_LENGTH / 2));

        game.grid[center.0 - 1][center.1 - 1] = true;
        game.grid[center.0][center.1 - 1] = true;
        game.grid[center.0 + 1][center.1 - 1] = true;
        game.grid[center.0 - 1][center.1] = true;
        game.grid[center.0 + 1][center.1] = true;
        game.grid[center.0 - 1][center.1 + 1] = true;
        game.grid[center.0][center.1 + 1] = true;
        game.grid[center.0 + 1][center.1 + 1] = true;

        assert_eq!(game.count_neighbors(center.0, center.1), 8);
    }

    #[test]
    fn test_count_neighbors_smallest_row() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (0, (COLUMN_LENGTH / 2));

        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0 + 1][cell.1 + 1] = true;

        assert_eq!(game.count_neighbors(cell.0, cell.1), 5);
    }

    #[test]
    fn test_count_neighbors_hightest_row() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH - 1, (COLUMN_LENGTH / 2));

        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0 - 1][cell.1 - 1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 + 1] = true;

        assert_eq!(game.count_neighbors(cell.0, cell.1), 5);
    }

    #[test]
    fn test_count_neighbors_smallest_column() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, 0);

        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0 + 1][cell.1] = true;
        game.grid[cell.0 + 1][cell.1 + 1] = true;

        assert_eq!(game.count_neighbors(cell.0, cell.1), 5);
    }

    #[test]
    fn test_count_neighbors_hightest_column() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH - 1);

        game.grid[cell.0 - 1][cell.1 - 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1 - 1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 + 1][cell.1] = true;

        assert_eq!(game.count_neighbors(cell.0, cell.1), 5);
    }

    #[test]
    fn test_rules_zero_neighbor() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }

    #[test]
    fn test_rules_one_neighbor() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0][cell.1 + 1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }

    #[test]
    fn test_rules_two_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;

        assert_eq!(game.rules(cell.0, cell.1), true);
    }

    #[test]
    fn test_rules_three_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1] = true;


        assert_eq!(game.rules(cell.0, cell.1), true);
    }

    #[test]
    fn test_rules_four_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 + 1][cell.1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }

    #[test]
    fn test_rules_five_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 - 1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }

    #[test]
    fn test_rules_six_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 - 1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1 - 1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }

    #[test]
    fn test_rules_seven_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 - 1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }

    #[test]
    fn test_rules_height_neighbors() {
        let mut game = Game::new(ROW_LENGTH, COLUMN_LENGTH);
        let cell = (ROW_LENGTH / 2, COLUMN_LENGTH / 2);

        game.grid[cell.0][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 - 1] = true;
        game.grid[cell.0 - 1][cell.1] = true;
        game.grid[cell.0 - 1][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 + 1] = true;
        game.grid[cell.0][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1 - 1] = true;
        game.grid[cell.0 + 1][cell.1] = true;
        game.grid[cell.0 + 1][cell.1 + 1] = true;

        assert_eq!(game.rules(cell.0, cell.1), false);
    }
}
