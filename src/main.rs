mod window;
mod graphics;

use window::{
    get_window,
    handle_message
};

fn main() {
    let graphics = graphics::Graphics::new();

    let mut window = get_window(graphics.get_width() as i32, graphics.get_height() as i32, "Game", "Game_Window").unwrap();

    graphics.assign_swap_chain(window.window_handle);

    loop {
        if !handle_message( &mut window ) {
            break;
        }
    }
}
