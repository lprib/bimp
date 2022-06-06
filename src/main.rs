use nannou::prelude::*;

struct Model {
    window: window::Id,
    grid: Grid<Tile, 64, 64>,
    rules: Vec<ReplacementRule<Tile, 3>>,
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
struct PatchOrientation {
    rotation_times: usize,
    position: (isize, isize),
}

struct ReplacementRule<T, const S: usize> {
    find: Grid<Option<T>, S, S>,
    replace: Grid<Option<T>, S, S>,
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

    fn get_patch_matches<const S: usize>(
        &self,
        patch: &Grid<Option<T>, S, S>,
    ) -> Vec<PatchOrientation> {
        let mut matches = Vec::new();
        for rotation_times in [0, 1, 2, 3] {
            let rotated_patch = patch.rotate(rotation_times);
            for offset_x in (-(S as isize - 1))..W as isize {
                for offset_y in (-(S as isize - 1))..H as isize {
                    if self.check_patch_at(&rotated_patch, offset_x, offset_y) {
                        matches.push(PatchOrientation {
                            rotation_times,
                            position: (offset_x, offset_y),
                        });
                    }
                }
            }
        }
        matches
    }

    fn replace_at<const S: usize>(
        &mut self,
        replacement_patch: &Grid<Option<T>, S, S>,
        orientation: &PatchOrientation,
    ) {
        let rotated = replacement_patch.rotate(orientation.rotation_times);
        // TODO abstract 2d iteration out of Grid
        for (y, row) in rotated.items.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if let Some(item) = item {
                    self.items[((y as isize) + orientation.position.1) as usize]
                        [((x as isize) + orientation.position.0) as usize] = *item;
                }
            }
        }
    }

    fn single_random_replace<const S: usize>(&mut self, rule: &ReplacementRule<T, S>) -> bool {
        let matches = self.get_patch_matches(&rule.find);
        if matches.is_empty() {
            return false;
        }
        let chosen_match = &matches[random::<usize>() % matches.len()];
        self.replace_at(&rule.replace, chosen_match);
        return true;
    }

    fn priority_random_repace<const S: usize>(&mut self, rules: &[ReplacementRule<T, S>]) {
        for rule in rules {
            if self.single_random_replace(rule) {
                break;
            }
        }
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
            1 => self.transform_indices(|_, y, size| size - 1 - y, |x, _, size| x),
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
                )
                .pad(tile_w / 10.0);

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
        .key_pressed(key_pressed_fn)
        .view(view)
        .build()
        .unwrap();

    let mut grid: Grid<Tile, 64, 64> = Default::default();
    grid.items[32][32] = Tile::Red;

    const R: Option<Tile> = Some(Tile::Red);
    const K: Option<Tile> = Some(Tile::Black);
    const W: Option<Tile> = Some(Tile::White);
    const G: Option<Tile> = Some(Tile::Green);
    const O: Option<Tile> = Some(Tile::Orange);
    const B: Option<Tile> = Some(Tile::Blue);
    const X: Option<Tile> = None;

    Model {
        window,
        grid,
        rules: vec![
            ReplacementRule {
                find: Grid {
                    items: [[R, K, K], [X, X, X], [X, X, X]],
                },
                replace: Grid {
                    items: [[W, W, R], [X, X, X], [X, X, X]],
                },
            },
            ReplacementRule {
                find: Grid {
                    items: [[R, K, W], [X, X, X], [X, X, X]],
                },
                replace: Grid {
                    items: [[G, W, O], [X, X, X], [X, X, X]],
                },
            },
            ReplacementRule {
                find: Grid {
                    items: [[O, W, G], [X, X, X], [X, X, X]],
                },
                replace: Grid {
                    items: [[O, K, B], [X, X, X], [X, X, X]],
                },
            },
            ReplacementRule {
                find: Grid {
                    items: [[B, W, W], [X, X, X], [X, X, X]],
                },
                replace: Grid {
                    items: [[K, K, B], [X, X, X], [X, X, X]],
                },
            },
            ReplacementRule {
                find: Grid {
                    items: [[B, W, O], [X, X, X], [X, X, X]],
                },
                replace: Grid {
                    items: [[K, K, R], [X, X, X], [X, X, X]],
                },
            },
        ],
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn key_pressed_fn(_app: &App, model: &mut Model, _k: Key) {
    model.grid.priority_random_repace(&model.rules)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Tile::LightGrey.color());

    model.grid.draw(&draw, app.window_rect().pad(20.0));

    draw.to_frame(app, &frame).unwrap();
}
