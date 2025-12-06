use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    init_roto();

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

use roto::Runtime;

fn init_roto() {
    // Step 1: Create a runtime
    let rt = Runtime::new();

    // Step 2: Compile the script and check for type errors
    let result = rt.compile("script.roto");
    let mut pkg = match result {
        Ok(pkg) => pkg,
        Err(err) => {
            panic!("{err}");
        }
    };

    // Step 3: Extract the function
    let func = pkg
        .get_function::<(), fn(i32) -> i32>("times_two")
        .unwrap();

    // Step 4: Call the function
    let result = func.call(&mut (), 4);
    println!("times_two(4) = {result}");
}