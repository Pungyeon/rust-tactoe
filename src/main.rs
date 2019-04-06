
use std::io;
use std::fmt;

#[test]
fn test_start_game() {
    assert_eq!(Game::new(Player::O).player, Player::O);
    assert_eq!(Game::new(Player::O).state(), State::ONGOING);
    assert_eq!(Game::new(Player::O).moves_left(), 9);
}

#[test]
fn test_player_changes_on_first_move() {
    assert_eq!(Game::new(Player::O)
        .play(TOP_LEFT).unwrap()
        .player, Player::X)
}

#[test]
fn test_player_changes_on_second_move() {
    assert_eq!(Game::new(Player::O)
        .play(TOP_LEFT).unwrap()
        .play(TOP_MID).unwrap()
        .player, Player::O);
}

#[test]
fn test_game_winning_state() {
    assert_eq!(Game::new(Player::O)
        .play(TOP_LEFT).unwrap()
        .play(BOT_LEFT).unwrap()
        .play(TOP_MID).unwrap()
        .play(BOT_MID).unwrap()
        .play(TOP_RIGHT).unwrap()
        .state(), State::WINNER);
}

#[test]
fn test_play_on_board() {
    let board = Board::new();

    assert_eq!(
        [Player::O, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE],
        board.put(Player::O, TOP_LEFT).unwrap().grid);
}

#[test]
fn test_play_on_board_twice() {
    let board = Board::new();

    assert_eq!(
        [Player::O, Player::X, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE],
        board.put(Player::O, TOP_LEFT).unwrap()
            .put(Player::X, TOP_MID).unwrap().grid);
}

#[test]
#[should_panic]
fn test_play_on_boar_cannot_overwrite() {
    let board = Board::new();

    board.put(Player::O, TOP_LEFT).unwrap()
        .put(Player::X, TOP_LEFT).unwrap();
}

#[derive(Copy, Clone)]
struct Board {
    grid: [Player; 9],
}

impl Board {
    fn new() -> Board {
        Board{
            grid: [Player::NONE; 9],
        }
    }

    fn put(mut self, player: Player, position: usize) -> Result<Board, String> {
        if position > 8 {
            return Err(format!("index out of range: {}", position))
        }
        if self.grid[position] != Player::NONE {
            return Err(format!("cannot overwrite position of already existing piece: position: {}, player: {:?}", position, self.grid[position]))
        }
        self.grid[position] = player;
        Ok(self)
    }
}

const TOP_LEFT : usize = 0;
const TOP_MID : usize = 1;
const TOP_RIGHT : usize = 2;
const MID_LEFT : usize = 3;
const MID_MID : usize = 4;
const MID_RIGHT : usize = 5;
const BOT_LEFT : usize = 6;
const BOT_MID : usize = 7;
const BOT_RIGHT : usize = 8;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Player {
    NONE, 
    O,
    X,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::NONE => write!(f, "-"),
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}

impl Player {
    fn opposite(&self) -> Player {
        match *self {
            Player::NONE => Player::NONE,
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    ONGOING,
    DRAW,
    WINNER,
}

#[derive(Copy, Clone)]
struct Game {
    player: Player,
    board: Board,
}

impl Game {
    fn new(player: Player) -> Game {
        Game{
            player: player,
            board: Board::new(),
        }
    }

    fn play(self, position: usize) -> Result<Game, String> {
        match self.board.put(self.player, position) {
            Ok(board) => Ok(self.change_player(board)),
            Err(e) => Err(e),
        }
    }

    fn change_player(self, board: Board) -> Game {
        match self.player {
            Player::NONE => Game{ player: Player::O, board: board },
            Player::O => Game{ player: Player::X, board: board },
            Player::X => Game{ player: Player::O, board: board },
        }
    }

    fn state(self) -> State {
        if self.board.grid[TOP_LEFT] != Player::NONE && self.board.grid[TOP_LEFT] == self.board.grid[TOP_MID] && self.board.grid[TOP_LEFT] == self.board.grid[TOP_RIGHT] ||
        self.board.grid[MID_LEFT] != Player::NONE && self.board.grid[MID_LEFT] == self.board.grid[MID_MID] && self.board.grid[MID_LEFT] == self.board.grid[MID_RIGHT] || 
        self.board.grid[BOT_LEFT] != Player::NONE && self.board.grid[BOT_LEFT] == self.board.grid[BOT_MID] && self.board.grid[BOT_LEFT] == self.board.grid[BOT_RIGHT] || 
        self.board.grid[TOP_LEFT] != Player::NONE && self.board.grid[TOP_LEFT] == self.board.grid[MID_LEFT] && self.board.grid[TOP_LEFT] == self.board.grid[BOT_LEFT] || 
        self.board.grid[TOP_MID] != Player::NONE && self.board.grid[TOP_MID] == self.board.grid[MID_MID] && self.board.grid[TOP_MID] == self.board.grid[BOT_MID] ||
        self.board.grid[TOP_RIGHT] != Player::NONE && self.board.grid[TOP_RIGHT] == self.board.grid[MID_RIGHT] && self.board.grid[TOP_RIGHT] == self.board.grid[BOT_RIGHT] ||
        self.board.grid[TOP_LEFT] != Player::NONE && self.board.grid[TOP_LEFT] == self.board.grid[MID_MID] && self.board.grid[TOP_LEFT] == self.board.grid[BOT_RIGHT] || 
        self.board.grid[TOP_RIGHT] != Player::NONE && self.board.grid[TOP_RIGHT] == self.board.grid[MID_MID] && self.board.grid[TOP_RIGHT] == self.board.grid[BOT_LEFT] {
            return State::WINNER;
        }

        if self.moves_left() == 0 {
            return State::DRAW;
        }

        return State::ONGOING;
    }

    fn moves_left(self) -> i32 {
        let mut moves_left = 0;
        for spot in self.board.grid.iter() {
            if spot == &Player::NONE {
                moves_left += 1;
            }
        }
        moves_left
    }

    fn print(self) {
        for i in 0..self.board.grid.len() {
            print!("{0: <20} ", self.board.grid[i]);
            if (i + 1) % 3 == 0 {
                print!("\n");
            }
        }
    }
}


fn main() {
    let mut game = Game::new(Player::O);

    while game.state() == State::ONGOING {
        let buffer = &mut String::new();
        match io::stdin().read_line(buffer) {
            Ok(txt) => {
                let position : usize = buffer.trim().parse().unwrap();
                game = game.play(position).unwrap();
            },
            Err(e) => println!("{}", e),
        }
        game.print();
    }

    match game.state() {
        State::DRAW => println!("Game Finished: DRAW"),
        State::WINNER => println!("Game Finishd: Winnder ({:?})", game.player.opposite()),
        State::ONGOING => println!("What the actual fuck?"),
    }
}
