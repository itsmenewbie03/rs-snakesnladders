const WINNING_POSITION: u8 = 100;
struct Player {
    name: String,
    position: u8,
    symbol: String,
}

impl Player {
    fn new(name: &'static str, symbol: &'static str) -> Player {
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

fn print_board(game_data: &Vec<Player>) {
    let mut out: Vec<String> = Vec::new();
    for x in (1..=WINNING_POSITION).rev() {
        if game_data.iter().any(|player| player.position == x) {
            let player = game_data
                .iter()
                .find(|player| player.position == x)
                .unwrap();
            out.push(format!("{:>4}", player.symbol));
        } else {
            out.push(format!("{:>4}", x));
            if x % 10 == 1 {
                if x / 10 % 2 == 0 {
                    out.reverse();
                }
                println!("{}", out.join(" "));
                out.clear();
            }
        }
    }
}
fn main() {
    let mut game_data: Vec<Player> = Vec::new();
    let mut player1 = Player::new("Player 1", "*");
    let mut player2 = Player::new("Player 2", "#");
    player1.move_player(6);
    player1.move_player(6);
    player1.move_player(6);
    player1.move_player(4);
    game_data.push(player1);
    game_data.push(player2);
    print_board(&game_data);
}

