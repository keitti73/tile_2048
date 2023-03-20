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
        for x in 0..4{
            print!("|");
            for y in 0..4{
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
}

fn main(){
    let mut new_game = Game::new();
    new_game.display()
}