use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Error {
    GameOver,
    AlreadyPlayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    X,
    O,
}

impl Team {
    fn to_str(&self) -> &str {
        match self {
            Team::X => "X",
            Team::O => "O",
        }
    }
}

#[derive(Debug)]
pub struct Tictactoe {
    board: Vec<Option<Team>>,
    width: usize,
    height: usize,
    pub cur_team: Team,
}

impl Tictactoe {
    pub fn new(width: usize, height: usize) -> Self {
        let board: Vec<Option<Team>> = vec![None; width * height];
        Tictactoe {
            board,
            width,
            height,
            cur_team: Team::X,
        }
    }

    pub fn play_move(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if let Some(team) = self.winner() {
            return Err(Error::GameOver);
        }

        let i = (y * self.width) + x;
        if let Some(space) = self.board.get_mut(i) {
            if space.is_some() {
                return Err(Error::AlreadyPlayed);
            }
            *space = Some(self.cur_team.clone());
            self.cur_team = if self.cur_team == Team::X {
                Team::O
            } else {
                Team::X
            };
        }
        Ok(())
    }

    pub fn winner(&mut self) -> Option<Team> {
        let all_same = |v: &[Option<Team>]| -> bool {
            v.get(0)
                .map(|f| v.iter().all(|i| i == f && i.is_some()))
                .unwrap_or(false)
        };
        let rows: Vec<_> = self.get_rows();
        for row in rows {
            if all_same(row) {
                let team = row.get(0).unwrap().unwrap();
                return Some(team);
            }
        }
        let cols = self.get_cols();
        for col in cols {
            if all_same(&col) {
                let team = col.get(0).unwrap().unwrap();
                return Some(team);
            }
        }
        let diags = self.get_diags();
        for diag in diags {
            if all_same(&diag) {
                let team = diag.get(0).unwrap().unwrap();
                return Some(team);
            }
        }
        None
    }

    fn get_rows(&self) -> Vec<&[Option<Team>]> {
        self.board.chunks(self.width).collect()
    }

    fn get_diags(&self) -> Vec<Vec<Option<Team>>> {
        let mut row = 0;
        let mut diag = vec![];
        for col in 0..self.width {
            let idx = (row * self.width) + col;
            if let Some(t) = self.board.get(idx) {
                diag.push(t.clone());
            }
            row += 1;
        }
        let mut row = self.width;
        let mut diag2 = vec![];
        for col in 0..self.width {
            row -= 1;
            let idx = (row * self.width) + col;
            if let Some(t) = self.board.get(idx) {
                diag2.push(t.clone());
            }
        }
        vec![diag, diag2]
    }

    fn get_cols(&self) -> Vec<Vec<Option<Team>>> {
        let cols_len = self.board.len() / self.width;
        let mut cols = vec![];
        for i in 0..cols_len {
            if let Some(col) = self.get_col(i) {
                cols.push(col);
            }
        }
        cols
    }

    fn get_col(&self, col_n: usize) -> Option<Vec<Option<Team>>> {
        if col_n > self.board.len() / self.width {
            return None;
        }
        let mut col = vec![];
        for (i, team) in self.board.iter().enumerate() {
            if i % self.width == col_n {
                col.push(team.clone());
            }
        }
        Some(col)
    }
}

impl Display for Tictactoe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let horizontal_line = "-----";
        let lines: Vec<_> = self.board.chunks(self.width).collect();
        let l_s: Vec<String> = lines
            .iter()
            .map(|line| -> String {
                let l: Vec<_> = line
                    .iter()
                    .map(|team| team.map_or(" ".to_string(), |t| t.to_str().to_owned()))
                    .collect();
                l.join("|")
            })
            .collect();
        let board = l_s.join(&format!("\n{}\n", horizontal_line));
        write!(f, "{}", board)
    }
}
