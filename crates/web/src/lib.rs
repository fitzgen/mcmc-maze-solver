use futures::prelude::*;
use mcmc_maze_solver::{Cell, Maze, Path};
use rand::SeedableRng;
use std::cmp;
use wasm_bindgen::{prelude::*, JsCast};

const GRID_COLOR: &str = "black";
const SAMPLE_COLOR: &str = "rgba(255,128,128,0.25)";
const SOLUTION_COLOR: &str = "blue";

const ROWS: u32 = 15;
const COLS: u32 = 30;

const PX_PER_CELL: u32 = 16;
const WIDTH: u32 = COLS * PX_PER_CELL + COLS + 1;
const HEIGHT: u32 = ROWS * PX_PER_CELL + ROWS + 1;

const DRAW_EVERY_N_MS: f64 = 50.0;

fn window() -> web_sys::Window {
    web_sys::window().expect("failed to get window")
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("failed to initialize logging");

    let document = window().document().expect("failed to get document");

    let ticks = document
        .get_element_by_id("ticks")
        .expect("failed to get #ticks");

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

    wasm_bindgen_futures::spawn_local(solve_maze(context, rng, ticks, maze));

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
    for to in maze.edges(cell) {
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

fn next_animation_frame() -> wasm_bindgen_futures::JsFuture {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        window()
            .request_animation_frame(&resolve)
            .expect("failed to request animation frame");
    });
    wasm_bindgen_futures::JsFuture::from(promise)
}

async fn solve_maze(
    context: web_sys::CanvasRenderingContext2d,
    mut rng: rand::rngs::SmallRng,
    ticks: web_sys::Element,
    maze: Maze,
) {
    let start_cell = Cell { row: 0, col: 0 };
    let destination_cell = Cell {
        row: maze.rows() - 1,
        col: maze.cols() - 1,
    };

    let performance = window()
        .performance()
        .expect("failed to get window.performance");

    let start = performance.now();
    let mut last_time = start;

    let mut i = 0;
    let mut is_new_animation_frame = true;
    let solution = mcmc_maze_solver::solve_maze(
        &mut rng,
        &maze,
        start_cell,
        destination_cell,
        |maze_and_path| {
            i += 1;

            if is_new_animation_frame {
                context.begin_path();
                context.set_stroke_style(&SAMPLE_COLOR.into());
                context.set_line_width(window().device_pixel_ratio());
                is_new_animation_frame = false;
            }

            if let Some((maze, path)) = maze_and_path {
                draw_path(&context, maze, start_cell, path);
            }

            let now = performance.now();
            let this_quanta_time = now - last_time;
            if this_quanta_time >= DRAW_EVERY_N_MS {
                last_time = now;

                context.stroke();
                is_new_animation_frame = true;

                let tpms = (i as f64) / (now - start);
                ticks.set_text_content(Some(&format!(
                    "   ticks/ms: {:>10.2}\ntotal ticks: {:>10}\n total time: {:>10.0}",
                    tpms,
                    i,
                    now - start,
                )));

                Some(Box::new(next_animation_frame().map(|_| ())) as _)
            } else {
                None
            }
        },
    )
    .await;

    if !is_new_animation_frame {
        context.stroke();
    }

    log::debug!(
        "solution in {} ticks and {:.3} ms = {:#?}",
        i,
        performance.now() - start,
        solution
    );

    context.begin_path();
    context.set_stroke_style(&SOLUTION_COLOR.into());
    context.set_line_width(window().device_pixel_ratio());

    draw_path(&context, &maze, start_cell, &solution);

    context.stroke();
}

fn draw_path(context: &web_sys::CanvasRenderingContext2d, maze: &Maze, start: Cell, path: &Path) {
    context.move_to(
        (start.col * (PX_PER_CELL + 1) + PX_PER_CELL / 2 + 1) as f64,
        (start.row * (PX_PER_CELL + 1) + PX_PER_CELL / 2 + 1) as f64,
    );

    for cell in maze.follow_path(start, path) {
        context.line_to(
            (cell.col * (PX_PER_CELL + 1) + PX_PER_CELL / 2 + 1) as f64,
            (cell.row * (PX_PER_CELL + 1) + PX_PER_CELL / 2 + 1) as f64,
        );
    }
}
