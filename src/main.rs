struct Board {
    vec: Vec<Vec<bool>>,
    row: usize,
    col: usize,
}

impl Board {

    fn show(&self) {
        for y in 0..self.col {
            for x in 0..self.row {
                print!("{}   ", (self.vec[y][x] as i8));
            }
            print!("\n");
        }
    }

    fn set(&mut self, x: usize, y: usize, setting: bool) {
        
        let check = x <= self.row-1 && y <= self.col-1;

        if check {
            self.vec[y][x] = setting;
        } else {
            println!("ERROR: Cannot modify out of bounds coordinate!");
        }
    }
}

fn board_init(size: usize) -> Board {
    Board {
        vec: vec![vec![false; size]; size],
        row: size,
        col: size
    }
}

fn main() {

    //Create a board
    let mut board: Board = board_init(10);

    //See if we can modify it
    board.set(9,9,true);

    board.show()
}
