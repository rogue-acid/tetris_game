#![feature(linked_list_remove)]

use std::{
    time::{self, Duration, Instant},
    vec,
};

use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2,
    window::{Event, Key},
};

struct TetrisShape<'a> {
    rect: RectangleShape<'a>,
}

#[derive(Debug)]
struct TetrisBlock {
    color: Option<Color>,
}

trait Move_Line_Down {
    fn move_line_down(&mut self, line_index: usize);
}

trait ClearLine {
    fn clear_line(&mut self, _: usize);
}

impl ClearLine for Vec<Vec<TetrisBlock>> {
    fn clear_line(&mut self, line_index: usize) {
        if self.is_empty() {
            return;
        }

        for pos in (1..=line_index).rev() {
            self.swap(pos, pos - 1)
        }

        for block in self[0].iter_mut() {
            block.color = None;
        }
    }
}

impl Move_Line_Down for Vec<Vec<TetrisBlock>> {
    fn move_line_down(&mut self, line_index: usize) {
        self.swap(line_index, line_index + 1)
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

    window.set_framerate_limit(60);

    let mut vector_of_rows: Vec<Vec<TetrisBlock>> = Vec::new();

    let mut falling_shape_vector: Vec<Vec<TetrisShape>> = Vec::new();

    vector_of_rows.resize_with(22, || {
        let mut row = Vec::new();
        row.resize_with(10, || TetrisBlock { color: None });
        row
    });

    vector_of_rows[3][0].color = Some(Color::BLUE);
    vector_of_rows[4][0].color = Some(Color::BLACK);
    vector_of_rows[5][0].color = Some(Color::RED);

    for block in vector_of_rows[14].iter_mut() {
        block.color = Some(Color::BLUE);
    }
    for block in vector_of_rows[2].iter_mut() {
        block.color = Some(Color::BLUE);
    }
    for block in vector_of_rows[20].iter_mut() {
        block.color = Some(Color::BLUE);
    }

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Q, .. } => window.close(),
                _ => (),
            }
        }

        while let Some(complete_row_index) = get_completed_row_index(&mut vector_of_rows) {
            vector_of_rows.clear_line(complete_row_index);
        }

        window.clear(Color::rgb(220, 220, 220));
        for (y, row) in vector_of_rows.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let color = match block.color {
                    Some(color) => color,
                    None => Color::WHITE,
                };

                let mut rect = RectangleShape::new();
                rect.set_outline_color(Color::BLACK);
                rect.set_outline_thickness(1.0);
                rect.set_size((block_size.x as f32, block_size.y as f32));
                rect.set_fill_color(color);
                rect.set_position((
                    (x as u32 * block_size.x) as f32,
                    (y as u32 * block_size.y) as f32,
                ));

                window.draw(&rect);
            }
        }
        window.display();
    }
}

fn get_completed_row_index(grid: &Vec<Vec<TetrisBlock>>) -> Option<usize> {
    grid.iter()
        .position(|row| row.iter().all(|b| b.color.is_some()))
}
