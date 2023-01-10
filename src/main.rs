mod curses; use curses::*;
mod filter; use filter::*;
mod state; use state::*;
use rand::random;
fn main() {
    let mut toggle = true;
    clear_screen();
    hide_cursor();
    let mut filter_system = random_filters(9,3,3,2.0);
    let mut state_left = random_state();
    let mut state_right = random_state();
    let cells = 2000;
    loop {
        for f in &filter_system {
            match toggle {
                true => filter_state(&f, &state_left, &mut state_right),
                false =>filter_state(&f, &state_right, &mut state_left),
            }
            toggle = !toggle;
        }
        display(&state_left);
        if rand::random::<f64>() < 0.02 {
            filter_system = random_filters(rand::random::<usize>()%11 + 1,5,5,2.0*rand::random::<f64>()+0.5);
        }
        if rand::random::<f64>() < 0.01 {
            state_left = random_state();
            state_right = random_state();
        }
    }
}
