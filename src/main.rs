use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key},
};

fn main() {
    println!("Hello, world!");

    let mut window = RenderWindow::new(
        (675, 675),
        "tik_tak_toe",
        sfml::window::Style::TITLEBAR,
        &Default::default(),
    );

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Q, .. } => window.close(),
                _ => (),
            }
        }

        window.clear(Color::rgb(220, 220, 220));
        window.display();
    }
}
