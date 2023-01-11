mod curses; use curses::*;
mod filter; use filter::*;
mod state; use state::*;
use std::env;    

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut default_settings: Vec<usize> =vec![50,100,7,5,20];
    for i in 0..default_settings.len() {
        if i < args.len() - 1 {
            let parsed = match args[i+1].parse::<usize>() {
                Ok(u) => u,
                Err(_) => 0,
            };
            if parsed > 0 && i < default_settings.len() {
                default_settings[i] = parsed;
            }
        }
    }
    let rows = default_settings[0];
    let cols = default_settings[1];
    let filters = default_settings[2];
    let span = default_settings[3] as i32;
    let flux = default_settings[4];
    let mut toggle = true;
    clear_screen();
    hide_cursor();
    let mut filter_system = simple_random_filters(filters,span,span,2.0);
    let mut state_left = random_state(rows, cols);
    let mut state_right = random_state(rows, cols);
    cursor_to(1,1);
    print!("fflo {} {} {} {} {}", rows, cols, filters, span, flux);
    loop {
        for f in &filter_system {
            match toggle {
                true => filter_state(&f, &state_left, &mut state_right, rows, cols),
                false =>filter_state(&f, &state_right, &mut state_left, rows, cols),
            }
            toggle = !toggle;
        }
        display(&state_left, rows, cols);
        if rand::random::<usize>()%1000 < flux {
            filter_system = simple_random_filters(filters,span,span,1.0);
        }
        if rand::random::<usize>()%2000 < flux {
            state_left = random_state(rows, cols);
            state_right = random_state(rows, cols);
        }
    }
}
