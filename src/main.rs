use std::io;
struct Player {
    index: (i32, i32),
}
struct Game {
    player: Player,
    world: Vec<Vec<u32>>,
}
impl Player {
    fn new() -> Player {
        Player { index: (1, 1) }
    }
}
impl Game {
    fn new() -> Game {
        let player = Player::new();
        let mut vector = Vec::new();
        for i in 0..5 {
            let mut temp_vector: Vec<u32> = Vec::new();
            for j in 0..=5 {
                if player.index.0 == i && player.index.1 == j {
                    temp_vector.push(1)
                } else {
                    temp_vector.push(0);
                }
            }
            vector.push(temp_vector);
        }
        Game {
            player,
            world: vector,
        }
    }
    fn move_player(&mut self, index: &(i32, i32)) {
        self.player.index.0 += index.0;
        self.player.index.1 += index.1;
        let mut vector = Vec::new();
        for i in 0..5 {
            let mut temp_vector: Vec<u32> = Vec::new();
            for j in 0..=5 {
                if self.player.index.1 == i && self.player.index.0 == j {
                    temp_vector.push(1)
                } else {
                    temp_vector.push(0);
                }
            }
            vector.push(temp_vector);
        }
        self.world = vector;
    }
    fn print(&self) {
        println!("Game World");
        for i in 0..self.world.len() {
            let mut string = String::new();
            for j in 0..self.world[i].len() {
                string.push_str(&self.world[i][j].to_string());
                string.push_str(" ");
            }
            println!("{}", string);
        }
    }
}
fn main() {
    let mut game = Game::new();
    loop {
        game.print();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let values: Vec<&str> = guess.split_whitespace().collect();
        if values.len() == 0 {
            continue;
        }
        if values[0] == "w" {
            game.move_player(&(0, -1));
        } else if values[0] == "s" {
            game.move_player(&(0, 1));
        } else if values[0] == "a" {
            game.move_player(&(-1, 0));
        } else if values[0] == "d" {
            game.move_player(&(1, 0));
        }
        continue;
    }
}
