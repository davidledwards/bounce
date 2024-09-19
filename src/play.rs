//! Player for ball bouncing.

use crate::ansi;
use crate::opt::Options;
use random::Source;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::process;

#[derive(Hash, Eq, PartialEq)]
pub struct Point {
    pub row: u16,
    pub col: u16,
}

impl Point {
    fn new(row: u16, col: u16) -> Point {
        Point { row, col }
    }
}

#[derive(Copy, Clone)]
enum Barrier {
    Horizontal,
    Vertical,
    Corner,
}

impl Display for Barrier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Barrier::Horizontal => '-',
            Barrier::Vertical => '|',
            Barrier::Corner => '+',
        };
        write!(f, "{c}")
    }
}

#[derive(Copy, Clone)]
enum Trajectory {
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

pub struct Player {
    rows: u16,
    cols: u16,
    ball_char: char,
    ball_color: u8,
    line_color: u8,
    barriers: HashMap<Point, Barrier>,
    ball: Point,
    traj: Trajectory,
    out: String,
}

impl Player {
    pub fn new(rows: u16, cols: u16, opts: &Options) -> Player {
        let mut this = Player {
            rows,
            cols,
            ball_char: opts.ball_char,
            ball_color: opts.ball_color,
            line_color: opts.line_color,
            barriers: Player::generate(rows, cols, opts.lines),
            ball: Point::new(rows / 2, cols / 2),
            traj: Trajectory::RightDown,
            out: String::new(),
        };
        this.render();
        this
    }

    pub fn more(&self) -> bool {
        self.barriers.len() > 0
    }

    pub fn next(&mut self) {
        self.clear_ball();
        let traj = self.next_trajectory();
        (self.ball, self.traj) = self.next_position(traj);
        self.show_ball();
        self.draw();
    }

    fn next_trajectory(&mut self) -> Trajectory {
        use Trajectory::*;
        use Barrier::*;

        match self.barriers.remove(&self.ball) {
            Some(barrier) => match (self.traj, barrier) {
                (RightUp, Horizontal) => RightDown,
                (RightUp, Vertical) => LeftUp,
                (RightUp, Corner) => LeftDown,
                (RightDown, Horizontal) => RightUp,
                (RightDown, Vertical) => LeftDown,
                (RightDown, Corner) => LeftUp,
                (LeftDown, Vertical) => RightDown,
                (LeftDown, Horizontal) => LeftUp,
                (LeftDown, Corner) => RightUp,
                (LeftUp, Vertical) => RightUp,
                (LeftUp, Horizontal) => LeftDown,
                (LeftUp, Corner) => RightDown,
            }
            None => self.traj,
        }
    }

    fn next_position(&mut self, traj: Trajectory) -> (Point, Trajectory) {
        use Trajectory::*;

        let (row, col, traj) = match traj {
            RightUp => {
                match (self.ball.row, self.ball.col) {
                    // top-right corner
                    (0, col) if col == self.cols - 1 => (1, col - 1, LeftDown),

                    // right edge
                    (row, col) if col == self.cols - 1 => (row - 1, col - 1, LeftUp),

                    // top edge
                    (0, col) => (1, col + 1, RightDown),

                    // no collision
                    (row, col) => (row - 1, col + 1, RightUp)
                }
            }
            RightDown => {
                match (self.ball.row, self.ball.col) {
                    // bottom-right corner
                    (row, col) if row == self.rows - 1 && col == self.cols - 1 => (row - 1, col - 1, LeftUp),

                    // right edge
                    (row, col) if col == self.cols - 1 => (row + 1, col - 1, LeftDown),

                    // bottom edge
                    (row, col) if row == self.rows - 1 => (row - 1, col + 1, RightUp),

                    // no collision
                    (row, col) => (row + 1, col + 1, RightDown),
                }
            }
            LeftDown => {
                match (self.ball.row, self.ball.col) {
                    // bottom-left corner
                    (row, 0) if row == self.rows - 1 => (row - 1, 1, RightUp),

                    // left edge
                    (row, 0) => (row + 1, 1, RightDown),

                    // bottom edge
                    (row, col) if row == self.rows - 1 => (row - 1, col - 1, LeftUp),

                    // no collision
                    (row, col) => (row + 1, col - 1, LeftDown),
                }
            }
            LeftUp => {
                match (self.ball.row, self.ball.col) {
                    // top-left edge
                    (0, 0) => (1, 1, RightDown),

                    // left edge
                    (row, 0) => (row - 1, 1, RightUp),

                    // top edge
                    (0, col) => (1, col - 1, LeftDown),

                    // no collision
                    (row, col) => (row - 1, col - 1, LeftUp),
                }
            }
        };
        (Point::new(row, col), traj)
    }

    fn clear_ball(&mut self) {
        self.out.push_str(ansi::set_cursor(&self.ball).as_str());
        self.out.push_str(ansi::reset_color().as_str());
        self.out.push(' ');
    }

    fn show_ball(&mut self) {
        self.out.push_str(ansi::set_cursor(&self.ball).as_str());
        self.out.push_str(ansi::set_color(self.ball_color).as_str());
        self.out.push(self.ball_char);
    }

    fn draw(&mut self) {
        self.out.push_str(ansi::set_cursor_to(self.rows, 0).as_str());
        print!("{}", self.out);
        let _ = io::stdout().flush();
        self.out.clear();
    }

    fn render(&mut self) {
        self.out.push_str(ansi::clear_screen().as_str());
        for (p, barrier) in &self.barriers {
            self.out.push_str(ansi::set_cursor(p).as_str());
            self.out.push_str(ansi::set_color(self.line_color).as_str());
            self.out.push_str(format!("{barrier}").as_str());
        }
        self.draw();
    }

    fn generate(rows: u16, cols: u16, lines: u32) -> HashMap<Point, Barrier> {
        let mut barriers = HashMap::new();
        let mut rand = random::default(process::id() as u64);
        for _ in 0..lines {
            let row_start = rand.read::<u16>() % rows;
            let col_start = rand.read::<u16>() % cols;
            if rand.read::<usize>() % 2 == 0 {
                let col_end = col_start + rand.read::<u16>() % (cols - col_start);
                for col in col_start..col_end {
                    let p = Point::new(row_start, col);
                    let barrier = match barriers.get(&p) {
                        Some(Barrier::Vertical) => Barrier::Corner,
                        Some(b) => *b,
                        None => Barrier::Horizontal,
                    };
                    barriers.insert(p, barrier);
                }
            } else {
                let row_end = row_start + rand.read::<u16>() % (rows - row_start);
                for row in row_start..row_end {
                    let p = Point::new(row, col_start);
                    let barrier = match barriers.get(&p) {
                        Some(Barrier::Horizontal) => Barrier::Corner,
                        Some(b) => *b,
                        None => Barrier::Vertical,
                    };
                    barriers.insert(p, barrier);
                }
            };
        }
        barriers
    }
}
