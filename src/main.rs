use std::fmt;
use std::cmp;

const BOARD_SIZE: usize = 8;

struct Board {
    frog: u8,
    toad: u8,
    turn: u8
}

impl Board {
    fn is_terminal(&self) -> bool {
        generate_moves(&self).len() == 0
    } }

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(7);

        for i in (1..BOARD_SIZE).rev() {
            if self.frog & (1 << i) != 0 {
                out.push('F');
            } else if self.toad & (1 << i) != 0 {
                out.push('T');
            }
            else {
                out.push('x');
            }
        }

        write!(f, "{}", out)
    }
}

fn generate_moves(state: &Board) -> Vec<Board> {
    let f_bits = state.frog;
    let t_bits = state.toad;


    let mut possible_moves: Vec<Board> = Vec::new();

    if state.turn == 0 { // F turn
        for i in 2..BOARD_SIZE {
            if f_bits & (1 << i) != 0 {
                if ( (f_bits | t_bits) & (1 << (i - 1))) == 0{ // Case empty tile // watch out for i = 0 case
                    let new_bits = f_bits ^ (1 << i) ^ (1 << (i - 1));
                    possible_moves.push( Board { frog: new_bits,  toad: t_bits , turn:state.turn ^ 1});
                } else{ // Case Jump
                    if i >= 2 && (t_bits & (1 << (i - 1))) != 0 && ((f_bits | t_bits) & (1 << (i - 2))) == 0 {
                        let new_bits = f_bits ^ (1 << i) ^ (1 << (i - 2));
                        possible_moves.push(Board { frog: new_bits, toad: t_bits, turn: state.turn ^ 1 });
                    }
                }
            }
        }
    } else {       // T  turn
        for i in 1..(BOARD_SIZE - 1) {
            if t_bits & (1 << i) != 0 { // self-collision
                if ((f_bits | t_bits) & (1 << (i + 1))) == 0{ // Case empty tile // watch out for i = 0 case
                    let new_bits = t_bits ^ (1 << i) ^ (1 << (i + 1));
                    possible_moves.push( Board { frog: f_bits,  toad: new_bits, turn:state.turn ^ 1});
                } else { // Case Jump
                    if i <= 5 && (f_bits & (1 << (i + 1))) != 0 && ((f_bits | t_bits) & (1 << (i + 2))) == 0 {
                        let new_bits = t_bits ^ (1 << i) ^ (1 << (i + 2));
                        possible_moves.push(Board { frog: f_bits, toad: new_bits, turn: state.turn ^ 1 });
                    }
                }
            }
        }
        
    }

    possible_moves
}

fn minimax(state: Board) -> i8 {
    if state.is_terminal() {
        return if state.turn == 0 { -10 } else { 10 };
    }

    if state.turn == 0 { // from max
        let mut best_score = i8::MIN;
        for b in generate_moves(&state) {
            let score = minimax(b);
            best_score = cmp::max(best_score, score);
        }
        best_score
    } else { // toad min
        let mut best_score = i8::MAX;
        for b in generate_moves(&state) {
            let score = minimax(b);
            best_score = cmp::min(best_score, score);
        }
        best_score
    }
}

fn main() {
    const initial_state: Board = Board {
        frog: 0b11000000,
        toad: 0b00000110,
        turn: 0b00000000
    };

    if minimax(initial_state) > 0 {
        println!["Frog wins!"]
    } else{
        println!["Toad wins!"]
    }

}
