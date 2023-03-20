
#[derive(Copy,Clone)]
struct Tile{
    value : usize
}

#[derive(Copy,Clone)]
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
        let mut bord =[[Cell::None;4];4];
        let bord=Bord{
            bord : bord
        };
        
        let game = Game{
            bord:bord
        };
        game
    }

    fn display(&self){
        println!("+-------+");
        for x in 0..4{
            print!("|");
            for y in 0..4{
                match self.bord.bord[x][y]{
                    Cell::Tile(Tile { value })=> print!("{}|",value),
                    Cell::None => print!(" |")

                };
            }
            println!("");
        };
        println!("+-------+");
    }

    fn add_number(& mut self){
    }
}

fn main(){
    let new_game = Game::new();
    new_game.display()
}