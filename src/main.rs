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
}


fn main() {
    let mut game = Game::new(SIZE, SIZE);
    game.grid[SIZE / 2][SIZE /2] = true;

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
}
