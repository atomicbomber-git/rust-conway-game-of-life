extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;
use rand::Rng;

const WINDOW_TITLE: &str = "Game of Life";
const WINDOW_WIDTH_PIXELS: u32 = 640;
const WINDOW_HEIGHT_PIXELS: u32 = 480;

// 1280 x 536


const DEAD_COLOR: Color = [255.0, 100.0, 255.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const LIVING_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

const TILE_SIZE_PIXELS: u32 = 5;
const TILE_COL_COUNT: u32 = WINDOW_WIDTH_PIXELS / TILE_SIZE_PIXELS;
const TILE_ROW_COUNT: u32 = WINDOW_HEIGHT_PIXELS / TILE_SIZE_PIXELS;

type Tiles = [[bool; TILE_COL_COUNT as usize]; TILE_ROW_COUNT as usize];
type TileNeighborCounts = [[u32; TILE_COL_COUNT as usize]; TILE_ROW_COUNT as usize];

fn main() {
    println!(
        "Window size: {} x {}; Tile size: {}, Row Count: {}, Col Count: {}",
        WINDOW_WIDTH_PIXELS,
        WINDOW_HEIGHT_PIXELS,
        TILE_SIZE_PIXELS,
        TILE_ROW_COUNT,
        TILE_COL_COUNT,
    );

    let mut rng = rand::thread_rng();
    let mut tiles: Tiles = [[false; TILE_COL_COUNT as usize]; TILE_ROW_COUNT as usize];
    let mut living_neighbor_counts: TileNeighborCounts = [[0; TILE_COL_COUNT as usize]; TILE_ROW_COUNT as usize];


    let mut randomize_tile_states = |tiles: &mut Tiles, p: f64| {
        for row in tiles.iter_mut() {
            for tile in row.iter_mut() {
                *tile = rng.gen_bool(p)
            }
        }
    };

    let calculate_living_neighbor_counts = |tiles: &Tiles, living_neighbor_counts: &mut TileNeighborCounts, row_count: u32, col_count: u32| {
        for row in 0..row_count {
            for col in 0..col_count {
                let tile_val = |row: u32, col: u32| { tiles[row as usize][col as usize] as u32 };
                let neighbor_count = &mut living_neighbor_counts[row as usize][col as usize];

                *neighbor_count = 0;

                // Top row
                if row > 0 {
                    // Top left
                    if col > 0 {
                        *neighbor_count += tile_val(row - 1, col -1);
                    }

                    // Top center
                    *neighbor_count += tile_val(row - 1, col);

                    // Top right
                    if col < (col_count - 1) {
                        *neighbor_count += tile_val(row - 1, col + 1);
                    }
                }

                // The same row
                // Left
                if col > 0 {
                    *neighbor_count += tile_val(row, col - 1);
                }

                // Right
                if col < (col_count - 1) {
                    *neighbor_count += tile_val(row, col + 1);
                }

                // The row below
                if row < (row_count - 1) {
                    // Left
                    if col > 0 {
                        *neighbor_count += tile_val(row + 1, col - 1);
                    }

                    // Center
                    *neighbor_count += tile_val(row + 1, col);

                    // Right
                    if col < (col_count - 1) {
                        *neighbor_count += tile_val(row + 1, col + 1);
                    }
                }
            }
        }
    };

    let update_tiles = |tiles: &mut Tiles, living_neighbor_counts: &TileNeighborCounts, row_count: u32, col_count: u32| {
        for row in 0..row_count {
            for col in 0..col_count {
                let neighbor_count = living_neighbor_counts[row as usize][col as usize];
                let tile = &mut tiles[row as usize][col as usize];

                if *tile {
                    if (neighbor_count < 2) || (neighbor_count > 3) {
                        *tile = false
                    }
                } else {
                    if neighbor_count == 3 {
                        *tile = true
                    }
                }
            }
        }
    };

    let mut window: PistonWindow =
        WindowSettings::new(
            WINDOW_TITLE,
            [WINDOW_WIDTH_PIXELS as f64, WINDOW_HEIGHT_PIXELS as f64],
        ).exit_on_esc(true).build().unwrap();

    let mut cursor_pos: Option<[f64; 2]> = None;
    let mut draw = false;

    let mut run_game = false;


    while let Some(event) = window.next() {
        if let Some(pos) = event.mouse_cursor_args() {
            cursor_pos = Some(pos)
        }

        if let Some(button) = event.press_args() {
            if button == Button::Keyboard(Key::Space) {
                run_game = !run_game
            }

            if button == Button::Mouse(MouseButton::Left) {
                draw = true
            }
        }

        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false
            }
        }

        if run_game {
            calculate_living_neighbor_counts(&tiles, &mut living_neighbor_counts, TILE_ROW_COUNT, TILE_COL_COUNT);
            update_tiles(&mut tiles, &living_neighbor_counts, TILE_ROW_COUNT, TILE_COL_COUNT);
        }

        for row in 0..TILE_ROW_COUNT {
            for col in 0..TILE_COL_COUNT {
                let tile = &mut tiles[row as usize][col as usize];
                if draw {
                    if let Some(pos) = cursor_pos {
                        let x_start = (col * TILE_SIZE_PIXELS) as f64;
                        let x_end = x_start + TILE_SIZE_PIXELS as f64;
                        let y_start = (row * TILE_SIZE_PIXELS) as f64;
                        let y_end = y_start + TILE_SIZE_PIXELS as f64;

                        if (pos[0] > x_start) && (pos[0] < x_end) {
                            if (pos[1] > y_start) && (pos[1] < y_end) {
                                *tile = true;
                            }
                        }
                    }
                }
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear(LIVING_COLOR, graphics);

            for row in 0..TILE_ROW_COUNT {
                for col in 0..TILE_COL_COUNT {
                    draw_tiles(
                        &context,
                        graphics,
                        row,
                        col,
                        tiles[row as usize][col as usize],
                    );
                }
            }
        });
    }
}

fn draw_tiles<G: Graphics>(context: &Context, graphics: &mut G, i: u32, j: u32, is_living: bool) {
    let rect_def = [
        (j * TILE_SIZE_PIXELS) as f64,
        (i * TILE_SIZE_PIXELS) as f64,
        TILE_SIZE_PIXELS as f64,
        TILE_SIZE_PIXELS as f64
    ];

    let rect_fill_color = if is_living { LIVING_COLOR } else { DEAD_COLOR };

    let rect_fill = Rectangle::new(rect_fill_color);
    rect_fill.draw(rect_def, &Default::default(), context.transform, graphics);

    let rect_border = Rectangle::new_border(BORDER_COLOR, 0.7);
    rect_border.draw(rect_def, &Default::default(), context.transform, graphics);
}
