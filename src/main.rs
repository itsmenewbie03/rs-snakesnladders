use rand::Rng;
use std::collections::HashMap;

const WINNING_POSITION: u8 = 100;

enum TileType {
    Snake,
    Ladder,
}

struct Game {
    players: Vec<Player>,
    special_tiles: Vec<SpecialTile>,
}

struct Player {
    name: String,
    position: u8,
    symbol: String,
}

struct SpecialTile {
    position: u8,
    move_to: u8,
    tile_type: TileType,
}

impl Player {
    fn new(name: &str, symbol: &str) -> Player {
        Player {
            name: String::from(name),
            position: 0,
            symbol: String::from(symbol),
        }
    }

    fn take_steps(&mut self, roll: u8) {
        self.position += roll;
    }

    fn move_to(&mut self, position: u8) {
        self.position = position;
    }
}

impl SpecialTile {
    fn new(position: u8, move_to: u8, tile_type: TileType) -> SpecialTile {
        SpecialTile {
            position,
            move_to,
            tile_type,
        }
    }
}

impl Game {
    fn new(players: Vec<Player>, special_tiles: Vec<SpecialTile>) -> Game {
        Game {
            players,
            special_tiles,
        }
    }

    fn print_state(&self) {
        print_board(&self.players);
    }

    // TODO: refactor to make cleaner code
    fn play(&mut self) {
        let mut turn = 0;
        loop {
            let roll = roll_die();
            // INFO: prevents from going over 100
            if self.players[turn].position + roll > WINNING_POSITION {
                println!("{}", &format!("{:-^50}", "| TURN LOG |"));
                println!("{} rolled a {} and moved to position {}.\nBut that's too far, so they stay at position {}", self.players[turn].name, roll, self.players[turn].position + roll, self.players[turn].position);
                println!("{}", "-".repeat(51));
                turn = (turn + 1) % self.players.len();
                println!("{}", &format!("{:-^50}", "| CURRENT BOARD |"));
                self.print_state();
                println!("{}", "-".repeat(51));
                println!("{}", &format!("{:-^50}", "| PLAYER POSITIONS |"));
                for player in &self.players {
                    println!(
                        "{} ({}) is at position {}",
                        player.name, player.symbol, player.position
                    );
                }
                println!("{}", "-".repeat(51));
                println!();
                continue;
            }

            // INFO: check if the player is on a special tile
            if let Some(special_tile) = self
                .special_tiles
                .iter()
                .find(|tile| tile.position == self.players[turn].position + roll)
            {
                println!("{}", &format!("{:-^50}", "| TURN LOG |"));
                println!(
                    "{} rolled a {} and moved to position {}",
                    self.players[turn].name,
                    roll,
                    self.players[turn].position + roll
                );
                match special_tile.tile_type {
                    TileType::Snake => {
                        println!(
                            "But they got bitten by a snake went down to position {}",
                            special_tile.move_to
                        );
                        println!("{}", "-".repeat(51));
                        self.players[turn].move_to(special_tile.move_to);
                    }
                    TileType::Ladder => {
                        println!(
                            "Landed on a ladder and went up to position {}",
                            special_tile.move_to
                        );
                        println!("{}", "-".repeat(51));
                        self.players[turn].move_to(special_tile.move_to);
                    }
                }
                turn = (turn + 1) % self.players.len();
                println!("{}", &format!("{:-^50}", "| CURRENT BOARD |"));
                self.print_state();
                println!("{}", "-".repeat(51));
                println!("{}", &format!("{:-^50}", "| PLAYER POSITIONS |"));
                for player in &self.players {
                    println!(
                        "{} ({}) is at position {}",
                        player.name, player.symbol, player.position
                    );
                }
                println!("{}", "-".repeat(51));
                println!();
                continue;
            }

            self.players[turn].take_steps(roll);

            println!("{}", &format!("{:-^50}", "| TURN LOG |"));
            println!(
                "{} rolled a {} and moved to position {}",
                self.players[turn].name, roll, self.players[turn].position
            );
            println!("{}", "-".repeat(51));
            if self.players[turn].position == WINNING_POSITION {
                println!("{} wins!", self.players[turn].name);
                break;
            }
            turn = (turn + 1) % self.players.len();
            println!("{}", &format!("{:-^50}", "| CURRENT BOARD |"));
            self.print_state();
            println!("{}", "-".repeat(51));
            println!("{}", &format!("{:-^50}", "| PLAYER POSITIONS |"));
            for player in &self.players {
                println!(
                    "{} ({}) is at position {}",
                    player.name, player.symbol, player.position
                );
            }
            println!("{}", "-".repeat(51));
            println!();
        }
    }
}

fn roll_die() -> u8 {
    rand::thread_rng().gen_range(1..=6)
}

fn print_board(game_data: &Vec<Player>) {
    let mut out: Vec<String> = Vec::new();
    for x in (1..=WINNING_POSITION).rev() {
        // println!("DEBUG: out is {} while x is {}", out.join("|"), x);
        if game_data.iter().any(|player| player.position == x) {
            let player = game_data
                .iter()
                .find(|player| player.position == x)
                .unwrap();
            // TODO: refactor for readability
            out.push(format!(
                "{}",
                // NOTE: this prints the player symbol along with a \n
                // if the position is in the last column
                // we are doing this since we use x % 10 == 1 to split the lines
                format!("{:>4}", player.symbol)
            ))
        } else {
            out.push(format!("{:>4}", x));
        }
        if x % 10 == 1 {
            if x / 10 % 2 == 0 {
                out.reverse();
            }
            println!("{}", out.join(" "));
            out.clear();
        }
    }
}
fn main() {
    let mut players: Vec<Player> = Vec::new();
    let mut special_tiles: Vec<SpecialTile> = Vec::new();

    // INFO : player initializations
    let player1 = Player::new("Player 1", "*");
    let player2 = Player::new("Player 2", "#");
    let player3 = Player::new("Player 3", "$");

    // INFO : special tile initializations
    let snakes: HashMap<u8, u8> =
        HashMap::from([(37, 3), (48, 16), (28, 10), (75, 32), (94, 71), (96, 42)]);

    let ladders: HashMap<u8, u8> = HashMap::from([(4, 56), (14, 55), (12, 50), (41, 79), (54, 88)]);

    for (position, move_to) in snakes {
        special_tiles.push(SpecialTile::new(position, move_to, TileType::Snake));
    }

    for (position, move_to) in ladders {
        special_tiles.push(SpecialTile::new(position, move_to, TileType::Ladder));
    }

    players.push(player1);
    players.push(player2);
    players.push(player3);

    let mut game = Game::new(players, special_tiles);
    game.play();
}
