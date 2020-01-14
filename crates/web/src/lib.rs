use mcmc_maze_solver::{Cell, Maze};
use rand::SeedableRng;
use std::cmp;
use wasm_bindgen::{prelude::*, JsCast};

const GRID_COLOR: &str = "black";

const ROWS: u32 = 30;
const COLS: u32 = 40;

const PX_PER_CELL: u32 = 16;
const WIDTH: u32 = COLS * PX_PER_CELL + COLS + 1;
const HEIGHT: u32 = ROWS * PX_PER_CELL + ROWS + 1;

fn window() -> web_sys::Window {
    web_sys::window().expect("failed to get window")
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("failed to initialize logging");

    let document = window().document().expect("failed to get document");

    let canvas = document
        .get_element_by_id("canvas")
        .expect("failed to get #canvas");
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("failed to cast to HtmlCanvasElement");

    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);

    let context = canvas
        .get_context("2d")?
        .expect("failed to get 2d canvas context");
    let context = context
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("2d canvas context is not a CanvasRenderingContext2d");

    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
    let maze = Maze::new(&mut rng, ROWS, COLS);
    draw_maze(&context, &maze);

    wasm_bindgen_futures::spawn_local(solve_maze(context, maze));

    Ok(())
}

fn draw_maze(context: &web_sys::CanvasRenderingContext2d, maze: &Maze) {
    context.clear_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);

    draw_grid(context);

    for cell in maze.cells() {
        draw_cell(context, maze, cell);
    }
}

fn draw_grid(context: &web_sys::CanvasRenderingContext2d) {
    context.begin_path();
    context.set_stroke_style(&GRID_COLOR.into());
    context.set_line_width(window().device_pixel_ratio());

    // Vertical lines.
    for row in 0..=ROWS {
        context.move_to(0.0, (row * (PX_PER_CELL + 1) + 1) as f64);
        context.line_to(
            ((PX_PER_CELL + 1) * COLS + 1) as f64,
            (row * (PX_PER_CELL + 1) + 1) as f64,
        );
    }

    // Horizontal lines.
    for col in 0..=COLS {
        context.move_to((col * (PX_PER_CELL + 1) + 1) as f64, 0.0);
        context.line_to(
            (col * (PX_PER_CELL + 1) + 1) as f64,
            ((PX_PER_CELL + 1) * COLS + 1) as f64,
        );
    }

    context.stroke();
}

fn draw_cell(context: &web_sys::CanvasRenderingContext2d, maze: &Maze, cell: Cell) {
    log::debug!("draw_call({:?})", cell);
    for to in maze.edges(cell) {
        log::debug!("    to = {:?}", to);
        let min_x = cmp::min(
            cell.col * (PX_PER_CELL + 1) + 2,
            to.col * (PX_PER_CELL + 1) + 2,
        );
        let max_x = cmp::max(
            cell.col * (PX_PER_CELL + 1) + PX_PER_CELL + 1,
            to.col * (PX_PER_CELL + 1) + PX_PER_CELL + 1,
        );
        let min_y = cmp::min(
            cell.row * (PX_PER_CELL + 1) + 2,
            to.row * (PX_PER_CELL + 1) + 2,
        );
        let max_y = cmp::max(
            cell.row * (PX_PER_CELL + 1) + PX_PER_CELL + 1,
            to.row * (PX_PER_CELL + 1) + PX_PER_CELL + 1,
        );
        context.clear_rect(
            min_x as f64,
            min_y as f64,
            (max_x - min_x) as f64,
            (max_y - min_y) as f64,
        );
    }
}

async fn solve_maze(context: web_sys::CanvasRenderingContext2d, maze: Maze) {
    unimplemented!()
}
