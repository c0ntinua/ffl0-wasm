use crate::curses::*;
use crate::filter::Filter;
use crate::pancurses::*;
use libm::tanh;
pub const ROWS : usize = 70;
pub const COLS : usize = 200;

pub type State = Vec<Vec<f64>>;

pub fn random_state() -> State {
    let mut state = vec![ vec![0f64;COLS ] ; ROWS ];
    for row in 0..ROWS {
        for col in 0..COLS {
            state[row][col] = 1.0 - 2.0*rand::random::<f64>();
        }
    }
    state
}
pub fn display(state : &State) -> () {
    //switch_to_alternate_buffer();
    for row in 0..ROWS {
        cursor_to(row + 1, 1);
        for col in 0..COLS {
            let brightness = hue(state[row][col]);
            set_color(brightness, brightness,brightness);
            //curses::cursor_to(row, col);
            print!("\u{2588}");
        }
    }
    
}
pub fn display_alt(window : &mut Window, state : &State) -> () {
    //switch_to_alternate_buffer();
    for row in 0..ROWS {
        //cursor_to(row + 1, 1);
        for col in 0..COLS {
            let brightness = hue(state[row][col]);
            set_color(brightness, brightness,brightness);
            window.mv(row as i32, col as i32);
            if brightness > 128 { window.addstr("\u{2588}");} else { window.addstr(" ");}
            //print!("\u{2588}");
        }
    }
    window.refresh();
    
}
pub fn hue(x : f64) -> u8 {
    (128.0 * (x + 1.0)).trunc() as u8

}

pub fn filter_state_cell(filter : &Filter, read_state : &State, center_row : i32, center_col : i32) -> f64 {
    let mut sum = 0f64;
    for touch in filter {
        let row = cyclical_index(touch.row + center_row, ROWS as i32);
        let col = cyclical_index(touch.col + center_col, COLS as i32);
        sum += touch.value*read_state[row as usize][col as usize];
    }
    tanh(sum)
}



pub fn filter_state(filter : &Filter, read_state : &State, write_state : &mut State) {
    for row in 0..ROWS {
        for col in 0..COLS {
            write_state[row][col] = filter_state_cell(filter, read_state, row as i32, col as i32);

        }
    }
}
pub fn filter_state_random_cell(filter : &Filter, read_state : &State, write_state : &mut State, n : usize) {
    for _ in 0..n {
        let row = rand::random::<usize>() % ROWS;
        let col = rand::random::<usize>() % COLS;
        write_state[row][col] = filter_state_cell(filter, read_state, row as i32, col as i32);
    }      
}

pub fn cyclical_index(i : i32, limit : i32) -> i32 {
    if i < 0 { return i + limit; }
    if i >= limit { return i - limit };
    i
}