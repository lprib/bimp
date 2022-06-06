use nannou::prelude::*;

struct Model {
    window: window::Id,
    test_grid: Grid<Tile, 4, 4>,
    rot: usize,
}

#[derive(Copy, Clone)]
enum Tile {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGrey,
    LightGrey,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Lavender,
    Pink,
    LightPeach,
}

trait Colorable {
    fn color(&self) -> Rgb<u8>;
}

/// If T implements Default, Option<T> implements Colorable with color = Default::default().color()
impl<T: Colorable + Default> Colorable for Option<T> {
    fn color(&self) -> Rgb<u8> {
        match self {
            Some(c) => c.color(),
            None => {
                let def: T = Default::default();
                def.color()
            }
        }
    }
}

impl Colorable for Tile {
    fn color(&self) -> Rgb<u8> {
        match self {
            Tile::Black => Rgb::new(0, 0, 0),
            Tile::DarkBlue => Rgb::new(29, 43, 83),
            Tile::DarkPurple => Rgb::new(126, 37, 83),
            Tile::DarkGreen => Rgb::new(0, 135, 81),
            Tile::Brown => Rgb::new(171, 82, 54),
            Tile::DarkGrey => Rgb::new(95, 87, 79),
            Tile::LightGrey => Rgb::new(194, 195, 199),
            Tile::White => Rgb::new(255, 241, 232),
            Tile::Red => Rgb::new(255, 0, 77),
            Tile::Orange => Rgb::new(255, 163, 0),
            Tile::Yellow => Rgb::new(255, 236, 39),
            Tile::Green => Rgb::new(0, 228, 54),
            Tile::Blue => Rgb::new(41, 173, 255),
            Tile::Lavender => Rgb::new(131, 118, 156),
            Tile::Pink => Rgb::new(255, 119, 168),
            Tile::LightPeach => Rgb::new(255, 204, 170),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Black
    }
}

struct Grid<T, const W: usize, const H: usize> {
    items: [[T; W]; H],
}

/// Grid of "T: Default" itself also defined default, filling the entire grid
impl<T: Default + Copy, const W: usize, const H: usize> Default for Grid<T, W, H> {
    fn default() -> Self {
        Self {
            items: [[Default::default(); W]; H],
        }
    }
}

impl<T: Default + Copy, const S: usize> Grid<T, S, S> {
    fn rot_90(&self) -> Self {
        self.transform_indices(|_x, y, size| size - 1 - y, |x, _y, size| size - 1 - x)
    }

    // fn rot_180(&self) -> Self

    fn transform_indices<R1, R2>(&self, x_transform: R1, y_transform: R2) -> Self
    where
        R1: Fn(usize, usize, usize) -> usize,
        R2: Fn(usize, usize, usize) -> usize,
    {
        let mut ret: Self = Default::default();
        self.items.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, item)| {
                let new_x = x_transform(x, y, S);
                let new_y = y_transform(x, y, S);
                ret.items[new_y][new_x] = item.clone();
            })
        });
        ret
    }

    fn rotate(&self, times: usize) -> Self {
        match times {
            // 0 degrees (no-op)
            0 => self.transform_indices(|x, _, _| x, |_, y, _| y),
            // 90 degrees
            1 => self.transform_indices(|_, y, size| size - 1 - y, |x, _, size| size - 1 - x),
            // 180 degrees
            2 => self.transform_indices(|x, _, size| size - 1 - x, |_, y, size| size - 1 - y),
            // 270 degrees
            3 => self.transform_indices(|_, y, _| y, |x, _, size| size - 1 - x),
            // else
            n => self.rotate(n % 4),
        }
    }
}

impl<T: Colorable, const W: usize, const H: usize> Grid<T, W, H> {
    fn draw(&self, draw: &Draw, rect: Rect) {
        let x = rect.top_left()[0];
        let y = rect.top_left()[1];

        let tile_w = rect.w() / W as f32;
        let tile_h = rect.h() / H as f32;

        for (tile_y_int, row) in self.items.iter().enumerate() {
            for (tile_x_int, item) in row.iter().enumerate() {
                let corner_x = x + tile_x_int as f32 * tile_w;
                let corner_y = y - tile_y_int as f32 * tile_h;
                let tile_rect = Rect::from_corner_points(
                    [corner_x, corner_y],
                    [corner_x - tile_w, corner_y - tile_h],
                );

                draw.rect()
                    .xy(tile_rect.xy())
                    .wh(tile_rect.wh())
                    .color(item.color());
            }
        }
    }
}

fn main() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(256, 256)
        .mouse_pressed(mouse_pressed_fn)
        .view(view)
        .build()
        .unwrap();

    const X: Tile = Tile::Red;
    const I: Tile = Tile::Black;

    Model {
        window,
        test_grid: Grid {
            items: [[X, I, I, X], [I, I, I, I], [X, I, I, X], [I, X, X, I]],
        },
        rot: 0,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn mouse_pressed_fn(_app: &App, model: &mut Model, _button: MouseButton) {
    model.rot += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Tile::LightGrey.color());
    let rot_grid = model.test_grid.rotate(model.rot);
    rot_grid.draw(&draw, Rect::from_w_h(200.0, 200.0));

    draw.to_frame(app, &frame).unwrap();
}
