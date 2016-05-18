use std::cmp;

use raytracing::{Canvas, Color, RayTracer};

pub struct Renderer<T: RayTracer> {
    canvas: Canvas,
    sequence: Sequence,
    ray_tracer: T,
    image_buffer: Vec<Vec<Color>>,
    _output_filename: Option<String>,
    is_fully_renderered: bool,
}

impl<T> Renderer<T> where T: RayTracer {
    pub fn new_with_file_output(canvas: Canvas, ray_tracer: T, filename: &str) -> Renderer<T> {
        let mut image_buffer: Vec<Vec<Color>> = Vec::with_capacity(canvas.width() as usize + 1);
        for i in 0..canvas.height() {
            image_buffer.push(Vec::with_capacity(canvas.height() as usize + 1));
            for _ in 0..canvas.width() {
                image_buffer[i].push(Color::new(0.3, 0.3, 0.3));
            }
        }

        let sequence = Sequence::new(canvas.width(), canvas.height());
        Renderer {
            canvas: canvas,
            sequence: sequence,
            ray_tracer: ray_tracer,
            image_buffer: image_buffer,
            _output_filename: Some(String::from(filename)),
            is_fully_renderered: false,
        }
    }

    #[inline(always)]
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    #[inline(always)]
    pub fn image_buffer(&self) -> &Vec<Vec<Color>> {
        &self.image_buffer
    }

    pub fn is_fully_renderered(&self) -> bool {
        self.is_fully_renderered
    }

    pub fn render_one_more_ray(&mut self) {
        if let Some((x, y, step)) = self.sequence.next() {
            let (start, direction) = self.canvas.ray_for_pixel(x, y);

            let color = self.ray_tracer.shoot_ray(start, direction);

            for i in x..cmp::min(x + step, self.canvas.width()) {
                for j in y..cmp::min(y + step, self.canvas.height()) {
                    self.image_buffer[j][i] = color.clone();
                }
            }
        } else {
            self.is_fully_renderered = true;
        }
    }
}

struct Sequence {
    size: (usize, usize),
    step: usize,
    indices: (usize, usize),
}

impl Sequence {
    pub fn new(width: usize, height: usize) -> Sequence {
        Sequence {
            size: (width, height),
            step: cmp::min(cmp::max(width, height).next_power_of_two() / 2, 64),
            indices: (0, 0),
        }
    }
}

impl Iterator for Sequence {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.indices == (0, 0) {
            self.indices.0 += self.step;

            return Some((0, 0, self.step));
        }

        loop {
            match (self.indices.0, self.indices.1, self.step) {
                (_, _, 0) => return None,

                (x, y, step) if x < self.size.0 && y < self.size.1 => {
                    self.indices = (x + step, y);

                    if x % (step * 2) != 0 || y % (step * 2) != 0 {
                        return Some((x, y, step));
                    }
                },

                (_, y, step) if y < self.size.1 => {
                    self.indices = (0, y + step);
                },

                (x, _, step) if x < self.size.0 => {
                    self.indices = (0, 0);
                    self.step = step / 2;
                },

                _otherwise => return None,
            }
        }
    }
}

#[test]
fn works_for_a_2_by_2_grid() {
    let sequence: Vec<(usize, usize, usize)> = Sequence::new(2, 2).collect();

    assert_eq!(sequence, vec!(
        (0, 0, 1),
        (1, 0, 1),
        (0, 1, 1),
        (1, 1, 1),
    ));
}

#[test]
fn works_for_a_4_by_4_grid() {
    let sequence: Vec<(usize, usize, usize)> = Sequence::new(4, 4).collect();

    assert_eq!(sequence, vec!(
        (0, 0, 2),
        (2, 0, 2),
        (0, 2, 2),
        (2, 2, 2),
        (1, 0, 1),
        (3, 0, 1),
        (0, 1, 1),
        (1, 1, 1),
        (2, 1, 1),
        (3, 1, 1),
        (1, 2, 1),
        (3, 2, 1),
        (0, 3, 1),
        (1, 3, 1),
        (2, 3, 1),
        (3, 3, 1),
    ));
}

#[test]
fn works_for_a_6_by_5_grid() {
    let sequence: Vec<(usize, usize, usize)> = Sequence::new(6, 5).collect();

    assert_eq!(sequence, vec!(
        (0, 0, 4),
        (4, 0, 4),
        (0, 4, 4),
        (4, 4, 4),
        (2, 0, 2),
        (0, 2, 2),
        (2, 2, 2),
        (4, 2, 2),
        (2, 4, 2),
        (1, 0, 1),
        (3, 0, 1),
        (5, 0, 1),
        (0, 1, 1),
        (1, 1, 1),
        (2, 1, 1),
        (3, 1, 1),
        (4, 1, 1),
        (5, 1, 1),
        (1, 2, 1),
        (3, 2, 1),
        (5, 2, 1),
        (0, 3, 1),
        (1, 3, 1),
        (2, 3, 1),
        (3, 3, 1),
        (4, 3, 1),
        (5, 3, 1),
        (1, 4, 1),
        (3, 4, 1),
        (5, 4, 1),
    ));
}

#[test]
fn it_caps_the_steps_at_64() {
    let sequence = Sequence::new(512, 512);

    assert_eq!(sequence.next(), Some(0, 0, 64));
}
