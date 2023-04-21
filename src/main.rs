struct Board {
    vec: Vec<Vec<bool>>,
    row: usize,
    col: usize,
}

impl Board {

    /* Display board to terminal */
    fn show(&self) {
        for y in 0..self.row {
            for x in 0..self.col {
                print!("{}   ", (self.vec[y][x] as i8));
            }
            print!("\n");
        }
    }

    /* Given coordinates and a boolean, update
    that cell to that boolean */
    fn set(&mut self, x: usize, y: usize, setting: bool) {
        
        let check = x <= self.col-1 && y <= self.row-1;

        if check {
            self.vec[y][x] = setting;
        } else {
            println!("ERROR: Cannot modify out of bounds coordinate!");
        }
    }

    /* Given a set of coordinates, is that cell alive? */
    fn alive(&self, x: usize, y: usize) -> bool {
        
        let check = x <= self.col-1 && y <= self.row-1;

        if check {
            self.vec[y][x]
        } else {
            println!("ERROR: Cannot check out of bounds coordinate!");
            false
        }
    }

    /* Given a set of coordinates, how many alive neighbors does this cell have */
    fn neighbors(&self, x: usize, y: usize) -> usize {

        let check = x <= self.col-1 && y <= self.row-1;
        if check {

            let mut neighbors: usize = 0;

            //West neighbor
            if x != 0 {
                if self.vec[y][x-1] {neighbors += 1};
            }

            //Northwest neighbor
            if x != 0 && y != 0 {
                if self.vec[y-1][x-1] {neighbors += 1};
            }

            //North neighbor
            if y != 0 {
                if self.vec[y-1][x] {neighbors += 1};
            }

            //Northeast neighbor
            if y != 0 && x != self.col-1 {
                if self.vec[y-1][x+1] {neighbors += 1};
            }

            //East neighbor
            if x != self.col-1 {
                if self.vec[y][x+1] {neighbors += 1};
            }

            //Southeast neighbor
            if x != self.col-1 && y != self.row-1 {
                if self.vec[y+1][x+1] {neighbors += 1};
            }

            //South neighbor
            if y != self.row-1 {
                if self.vec[y+1][x] {neighbors += 1};
            }

            //Southwest neighbor
            if x != 0 && y != self.row-1 {
                if self.vec[y+1][x-1] {neighbors += 1};
            }

            neighbors

        } else {
            println!("ERROR: Cannot check out of bounds coordinate!");
            0
        }

    }

    /* Given the board's current state at time t, modify 
    the board to what it should look like at time t+1 */

    /*
    fn update_board(&mut self) {

        //Create a new board
        let mut new: Vec<Vec<bool>> = vec![vec![false; self.col]; self.row];

        for y in 0..self.row {
            for x in 0..self.col {
            }
        }
    }
    */
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
    board.set(0,0,true);
    board.set(9,0,true);
    board.set(0,9,true);

    board.set(8,8,true);
    board.set(0,1,true);
    board.set(8,0,true);
    board.set(0,8,true);

    //See if neighbors is working properly
    println!("Neighbors for 9,9: {}", board.neighbors(9,9));
    println!("Neighbors for 0,0: {}", board.neighbors(0,0));
    println!("Neighbors for 9,0: {}", board.neighbors(9,0));
    println!("Neighbors for 0,9: {}", board.neighbors(0,9));


    //See if alive works properly
    println!("Cell 0,0 is alive {}", board.alive(0,0));
    println!("Cell 0,8 is alive {}", board.alive(0,8));
    println!("Cell 5,5 is alive {}", board.alive(5,5));
    println!("Cell 1,2 is alive {}", board.alive(1,2));


    board.show()
}
