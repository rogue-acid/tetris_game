#![feature(linked_list_remove)]

use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2,
    window::{Event, Key},
};
struct TetrisShape<'a> {
    rect: RectangleShape<'a>,
}

struct TetrisBlock {
    color: Option<Color>,
}

trait ClearLine {
    fn clear_line(&mut self, _: usize);
}

impl<T> ClearLine for Vec<Vec<T>> {
    fn clear_line(&mut self, line_index: usize) {
        // for index in line_index..self.len() {
        //     println!("{}", index)
        // }
        // for (pos, row) in self.iter_mut().enumerate() {
        //     self[pos] = self[pos + 1];
        // }
    }
}

fn main() {
    let block_size = Vector2::<u32>::new(30, 30);
    let grid_size = Vector2::<u32>::new(10, 22);

    let window_size = block_size * grid_size;
    let mut window = RenderWindow::new(
        (window_size.x, window_size.y),
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

        let y = for (pos, value) in vector_of_rows.iter().enumerate() {
            let y = pos as i32;
        };

        let x = for row in vector_of_rows.iter() {
            for (pos, value) in row.iter().enumerate() {
                let x = pos;
            }
        };

        let color = Color::rgb(220, 120, 125);

        let mut rect = RectangleShape::new();
        rect.set_outline_color(Color::BLACK);
        rect.set_outline_thickness(1.0);
        rect.set_size((block_size.x as f32, block_size.y as f32));
        rect.set_fill_color(color);
        // rect.set_position((
        //     (x as u32 * block_size.x) as f32,
        //     (y as u32 * block_size.y) as f32,
        // ));

        window.clear(Color::rgb(220, 220, 220));
        window.draw(&rect);
        window.display();
    }
}
