use rand::Rng;

const WINNING_POSITION: u8 = 100;

struct Game {
    players: Vec<Player>,
}

struct Player {
    name: String,
    position: u8,
    symbol: String,
}

impl Player {
    fn new(name: &str, symbol: &str) -> Player {
        Player {
            name: String::from(name),
            position: 0,
            symbol: String::from(symbol),
        }
    }
    fn move_player(&mut self, roll: u8) {
        self.position += roll;
    }
}
impl Game {
    fn new(players: Vec<Player>) -> Game {
        Game { players }
    }

    fn print_state(&self) {
        print_board(&self.players);
    }

    // TODO: refactor to make cleaner code
    fn play(&mut self) {
        let mut turn = 0;
        loop {
            let roll = roll_die();
            // NOTE: prevents from going over 100
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
                continue;
            }
            self.players[turn].move_player(roll);

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
    let mut game_data: Vec<Player> = Vec::new();
    let player1 = Player::new("Player 1", "*");
    let player2 = Player::new("Player 2", "#");
    let player3 = Player::new("Player 3", "$");
    game_data.push(player1);
    game_data.push(player2);
    game_data.push(player3);
    let mut game = Game::new(game_data);
    game.play();
}
