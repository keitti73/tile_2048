use rand::Rng;

#[derive(Copy,Clone,PartialEq)]
struct Tile{
    value : usize
}

#[derive(Copy,Clone,PartialEq)]
enum Cell {
    Tile(Tile),
    None
}

impl Cell {
    fn output_value(self)-> usize{
        match self{
            Cell::None => return 0,
            Cell::Tile(Tile { value }) => value
        }
    }
}

struct Bord{
    bord : [[Cell;4];4]
}

struct Game{
    bord:Bord
}
impl Game{
    fn new()-> Self{
        let bord =[[Cell::None;4];4];
        let bord=Bord{
            bord : bord
        };
        
        let mut game = Game{
            bord:bord
        };

        game.add_number();
        game.add_number();

        game
    }

    fn display(&self){
        println!("+---+---+---+---+");
        for y in 0..4{
            print!("|");
            for x in 0..4{
                match self.bord.bord[x][y]{
                    Cell::Tile(Tile { value })=> print!(" {} |",value),
                    Cell::None => print!("   |")
                };
            }
            println!("");
            println!("+---+---+---+---+");
        };
    }

    fn add_number(& mut self){
        let mut i :usize = 0;
        for x in 0..4{
            for y in 0..4{
                if self.bord.bord[x][y]==Cell::None{
                    i = i+1;
                }
            }
        }
        if i == 0 {
            return ()
            //game over
        }

        loop{
            let rng = (rand::thread_rng().gen_range(1..4),rand::thread_rng().gen_range(1..4));

            if self.bord.bord[rng.0][rng.1] == Cell::None{
                self.bord.bord[rng.0][rng.1] = Cell::Tile(Tile { value: 2 });
                break;
            }
            else{
                continue
            }
        }
    }

    fn move_right(&mut self){
        for y in 0..4{
            for x in (0..4).rev(){
                for xx in 1..x{
                    if self.bord.bord[x][y] == Cell::None{
                        continue;
                    }
                    else if self.bord.bord[x][y] == self.bord.bord[x-xx][y]{
                            let mut value = self.bord.bord[x][y].output_value().clone();
                            value = value*2;
                            self.bord.bord[x][y] = Cell::Tile(Tile{value: value});
                            self.bord.bord[x-xx][y] = Cell::None
                    }
                }
            }
        };

        for y in 0..4{
            let mut i = 0;
            for x in (0..3).rev(){
                if self.bord.bord[x][y] == Cell::None{
                    continue;
                }
                else{
                    self.bord.bord[3-i][y] = self.bord.bord[x][y].clone();
                    self.bord.bord[x][y] = Cell::None;
                    i+=1
                }
            }
        }
        self.add_number()
    }

}

fn main(){
    let mut new_game = Game::new();
    new_game.display();
    new_game.move_right();
    new_game.display()
}