use crate::curses::*;
use crate::filter::Filter;
use libm::tanh;

pub type State = Vec<Vec<f64>>;

pub fn random_state(rows : usize, cols : usize) -> State {
    let mut state = vec![ vec![0f64;cols ] ; rows ];
    for row in 0..rows {
        for col in 0..cols {
            state[row][col] = 1.0 - 2.0*rand::random::<f64>();
        }
    }
    state
}
pub fn display(state : &State, rows : usize, cols : usize) -> () {
    for row in 0..rows {
        cursor_to(row + 2, 1);
        for col in 0..cols {
            let brightness = hue(state[row][col]);
            set_color(brightness, brightness,brightness);
            print!("\u{2588}");
        }
    }  
}
pub fn hue(x : f64) -> u8 {
    (128.0 * (x + 1.0)).trunc() as u8

}

pub fn filter_state_cell(filter : &Filter, read_state : &State, center_row : i32, center_col : i32,rows : usize, cols : usize) -> f64 {
    let mut sum = 0f64;
    for touch in filter {
        let row = cyclical_index(touch.row + center_row, rows as i32);
        let col = cyclical_index(touch.col + center_col, cols as i32);
        sum += touch.value*read_state[row as usize][col as usize];
    }
    tanh(sum)
}

pub fn filter_state(filter : &Filter, read_state : &State, write_state : &mut State, rows : usize, cols : usize ) {
    for row in 0..rows {
        for col in 0..cols {
            write_state[row][col] = filter_state_cell(filter, read_state, row as i32, col as i32, rows, cols);
        }
    }
}

pub fn cyclical_index(i : i32, limit : i32) -> i32 {
    if i < 0 { return i + limit; }
    if i >= limit { return i - limit };
    i
}