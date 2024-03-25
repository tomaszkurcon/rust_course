use std::io;
 
fn main() {
    let signs = ['X', 'O'];
    let mut board = [[' ';3];3];
    let players = create_players();
    println!("Gracz {}: {} Gracz {}: {}", players[0], signs[0], players[1], signs[1]);
    let mut turn = 0;
    let mut round:i8 = 0;
    loop {
        let mut input = String::new();
        println!("Gracz {} podaj pole:", players[turn]);
        let _ = io::stdin().read_line(&mut input);
        let field:i8 = input.trim().parse().expect("Not a number");
        if field < 1 || field > 9 {
            println!("Podana liczba {field} nie jest za zakresu 1-9. Podaj poprawną wartość.");
            continue
        }
        if draw_board(&mut board, field, signs[turn]) {
      
            if check_if_win(&mut board, field, signs[turn]) {
                println!("Gracz {} wygrał", players[turn]);
                break
            }
            turn = (turn + 1) % 2;
            round = round + 1;
            if round == 9 {
                println!("Remis");
                break
            }
        }
    
    }
}

fn create_players() -> [String; 2] {
    let mut player_one = String::new();
    let mut player_two = String::new();
    println!("Podaj imie gracza 1:");
    let _ = io::stdin().read_line(&mut player_one); 
    println!("Podaj imie gracza 2:");
    let _ = io::stdin().read_line(&mut player_two); 
    let players = [player_one.trim().to_string(), player_two.trim().to_string()];
    players
}

fn draw_board(board : &mut [[char; 3]; 3], field:i8, sign:char) -> bool {
    let row = (field - 1) / 3;
    let col = (field - 1) % 3;
    if board[row as usize][col as usize] != ' ' {
        println!("Podane pole jest już zajęte, podaj inne pole.");
        return false
    } 
    board[row as usize][col as usize] = sign;
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", board[i][j]);
        }
        println!();
    }
    true
}

fn check_if_win(board : &mut [[char; 3]; 3], field:i8,sign:char) -> bool {
   let col = (field - 1) % 3;
   let row = (field - 1) / 3;
   for i in 0..3 {
         if board[row as usize][i] != sign {
              break
         }
         if i == 2 {
              return true
         }
   }
    for i in 0..3 {
            if board[i][col as usize] != sign {
                  break
            }
            if i == 2 {
                  return true
            }
    }
    if row == col {
        for i in 0..3 {
            if board[i][i] != sign {
                break
            }
            if i == 2 {
                return true
            }
        }
    }
    if row + col == 2 {
        for i in 0..3 {
            if board[i][2-i] != sign {
                break
            }
            if i == 2 {
                return true
            }
        }
    }
    false

}
