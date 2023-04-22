use std::{thread, time};

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
                print!("{}  ", (self.vec[y][x] as i8));
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

    /* Given a set of coordinates, how many alive neighbours does this cell have */
    fn neighbours(&self, x: usize, y: usize) -> usize {

        let check = x <= self.col-1 && y <= self.row-1;
        if check {

            let mut neighbours: usize = 0;

            //West neighbor
            if x != 0 {
                if self.vec[y][x-1] {neighbours += 1};
            }

            //Northwest neighbor
            if x != 0 && y != 0 {
                if self.vec[y-1][x-1] {neighbours += 1};
            }

            //North neighbor
            if y != 0 {
                if self.vec[y-1][x] {neighbours += 1};
            }

            //Northeast neighbor
            if y != 0 && x != self.col-1 {
                if self.vec[y-1][x+1] {neighbours += 1};
            }

            //East neighbor
            if x != self.col-1 {
                if self.vec[y][x+1] {neighbours += 1};
            }

            //Southeast neighbor
            if x != self.col-1 && y != self.row-1 {
                if self.vec[y+1][x+1] {neighbours += 1};
            }

            //South neighbor
            if y != self.row-1 {
                if self.vec[y+1][x] {neighbours += 1};
            }

            //Southwest neighbor
            if x != 0 && y != self.row-1 {
                if self.vec[y+1][x-1] {neighbours += 1};
            }

           neighbours 

        } else {
            println!("ERROR: Cannot check out of bounds coordinate!");
            0
        }

    }

    /* Given the board's current state at time t, modify 
    the board to what it should look like at time t+1 */
    fn update_board(&mut self) -> bool {

        //Create a new board
        let mut new: Vec<Vec<bool>> = vec![vec![false; self.col]; self.row];

        for y in 0..self.row {
            for x in 0..self.col {

                //Things we need to know about every cell
                let neighbour_count = self.neighbours(x,y);
                let alive = self.alive(x,y);

                //Rule 1
                if alive && (neighbour_count == 2 || neighbour_count == 3) {
                    new[y][x] = true;

                //Rule 2
                } else if !alive && neighbour_count == 3 {
                    new[y][x] = true;

                //In all other cases the cell is just dead
                } else {
                    new[y][x] = false
                }
            }
        }

        //If we reach a game state where there is
        //no change, the game is over and we can
        //return false
        if self.vec == new {
            false
        
        //The general case is that the game is not over
        } else {
            self.vec = new;
            true
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

    //Create a "block"
    board.set(5,5,true);
    board.set(5,6,true);
    board.set(6,5,true);
    board.set(6,6,true);

    //Create a "blinker"
    board.set(2,1,true);
    board.set(2,2,true);
    board.set(2,3,true);

    let mut counter = 0;
    loop {

        println!("Counter t={counter}");
        board.show();

        //Update the board, and if it's false
        //break
        if !board.update_board() {
            break;
        }

        //Wait for a moment
        thread::sleep(time::Duration::from_millis(1000));
        counter += 1;
    }
}
