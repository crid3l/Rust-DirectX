mod window;

use window::{
    get_window,
    handle_message
};

fn main() {
    let mut window = get_window(800, 600, "Game", "Game_Window").unwrap();

    loop {
        if !handle_message( &mut window ) {
            break;
        }
    }
}
