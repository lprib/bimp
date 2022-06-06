use nannou::prelude::*;

struct Model {
    window: window::Id,
    test_grid: Grid<Tile, 4, 4>,
    rot: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug)]
struct PatchMatch {
    rotation_times: usize,
    position: (isize, isize),
}

impl<T: Eq + Copy, const W: usize, const H: usize> Grid<T, W, H> {
    fn check_patch_at<const S: usize>(
        &self,
        patch: &Grid<Option<T>, S, S>,
        offset_x: isize,
        offset_y: isize,
    ) -> bool {
        for (patch_y, row) in patch.items.iter().enumerate() {
            'inner: for (patch_x, item) in row.iter().enumerate() {
                match item {
                    // None is a 'dont care' value and matches anything
                    None => continue 'inner,
                    Some(item) => {
                        let grid_x = patch_x as isize + offset_x;
                        let grid_y = patch_y as isize + offset_y;
                        // patch has a value but is outside of the grid, BAD!
                        if grid_x < 0
                            || grid_y < 0
                            || grid_x >= (W as isize)
                            || grid_y >= (H as isize)
                        {
                            return false;
                        }
                        let grid_item = &self.items[grid_y as usize][grid_x as usize];
                        // if _any_ items fail to match, the whole patch fails
                        if grid_item != item {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn get_patch_matches<const S: usize>(&self, patch: &Grid<Option<T>, S, S>) -> Vec<PatchMatch> {
        let mut matches = Vec::new();
        for rotation_times in [0, 1, 2, 3] {
            let rotated_patch = patch.rotate(rotation_times);
            for offset_x in (-(S as isize - 1))..W as isize {
                for offset_y in (-(S as isize - 1))..H as isize {
                    println!("{}, {}", offset_x, offset_y);
                    if self.check_patch_at(&rotated_patch, offset_x, offset_y) {
                        matches.push(PatchMatch {
                            rotation_times,
                            position: (offset_x, offset_y),
                        });
                        println!("Match!");
                    }
                }
            }
        }
        matches
    }
}

/// Rotation only implemented for square grids (W==H)
impl<T: Default + Copy, const S: usize> Grid<T, S, S> {
    /// x_transform: lambda of (old_x, old_y, size) -> new_x
    /// y_transform: lambda of (old_x, old_y, size) -> new_y
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
                ret.items[new_y][new_x] = *item;
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
    //nannou::app(model).event(event).run();
    const B: Tile = Tile::Black;
    const R: Tile = Tile::Red;
    let grid = Grid {
        items: [
            [R, B, B, B, B, B, B, B],
            [R, B, R, R, B, B, B, B],
            [B, B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B, B],
        ],
    };

    const PD: Option<Tile> = None;
    const PR: Option<Tile> = Some(Tile::Red);

    let patch = Grid {
        items: [[PR, PR], [PD, PD]],
    };

    println!("{:#?}", grid.get_patch_matches(&patch));
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
