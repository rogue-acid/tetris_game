#![feature(linked_list_remove)]

use {
    sfml::{
        graphics::{Color, RectangleShape, RenderTarget, RenderWindow},
        window::{Event, Key},
    },
    std::collections::LinkedList,
};

struct TetrisShape<'a> {
    rect: RectangleShape<'a>,
}

struct TetrisBlock {
    color: Color,
}

fn main() {
    let mut window = RenderWindow::new(
        (675, 675),
        "Tetris",
        sfml::window::Style::TITLEBAR,
        &Default::default(),
    );

    let mut list_of_rows = LinkedList::<Vec<Option<TetrisBlock>>>::new();

    for _ in 0..22 {
        let mut row = Vec::new();
        row.resize_with(10, || None);
        list_of_rows.push_back(row);
    }

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
