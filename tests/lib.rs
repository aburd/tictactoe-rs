use tictactoe::{Error, Team, Tictactoe};

#[test]
fn board_is_displayed_correctly_with_no_moves() {
    let t = Tictactoe::new(3, 3);
    let new_board = r#" | | 
-----
 | | 
-----
 | | "#;
    assert_eq!(t.to_string(), new_board);
}

#[test]
fn board_is_displayed_correctly_with_one_move() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(2, 2);
    let new_board = r#" | | 
-----
 | | 
-----
 | |X"#;
    assert_eq!(t.to_string(), new_board);
}

#[test]
fn board_is_displayed_correctly_with_two_moves() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(2, 2);
    t.play_move(1, 2);
    let new_board = r#" | | 
-----
 | | 
-----
 |O|X"#;
    assert_eq!(t.to_string(), new_board);
}

#[test]
fn winner_in_top_row() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 0);
    t.play_move(0, 1);
    t.play_move(1, 0);
    let win = t.winner();
    assert_eq!(win, None);
    t.play_move(1, 1);
    t.play_move(2, 0);
    let win = t.winner();
    assert_eq!(win, Some(Team::X));
}

#[test]
fn playing_after_win_gives_error() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 0);
    t.play_move(0, 1);
    t.play_move(1, 0);
    t.play_move(1, 1);
    t.play_move(2, 0);
    let res = t.play_move(2, 1);
    assert_eq!(res, Err(Error::GameOver));
}

#[test]
fn playing_same_move() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 0);
    let res = t.play_move(0, 0);
    assert_eq!(res, Err(Error::AlreadyPlayed));
}

#[test]
fn winner_in_first_col() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 0);
    t.play_move(1, 0);
    t.play_move(0, 1);
    t.play_move(1, 1);
    t.play_move(0, 2);
    let win = t.winner();
    assert_eq!(win, Some(Team::X));
}

#[test]
fn winner_o_in_sec_col() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 0);
    t.play_move(1, 0);
    t.play_move(0, 1);
    t.play_move(1, 1);
    t.play_move(2, 0);
    t.play_move(1, 2);
    let win = t.winner();
    assert_eq!(win, Some(Team::O));
}

#[test]
fn winner_in_diag() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 0);
    t.play_move(1, 0);
    t.play_move(1, 1);
    t.play_move(0, 2);
    t.play_move(2, 2);
    let win = t.winner();
    assert_eq!(win, Some(Team::X));
}

#[test]
fn winner_in_second_diag() {
    let mut t = Tictactoe::new(3, 3);
    t.play_move(0, 2);
    t.play_move(1, 0);
    t.play_move(1, 1);
    t.play_move(0, 1);
    t.play_move(2, 0);
    let win = t.winner();
    assert_eq!(win, Some(Team::X));
}
