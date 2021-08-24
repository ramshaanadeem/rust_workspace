use piston_window::types::Color;
use piston_window::{rectangle, text, Context, G2d, Glyphs, Transformed};
extern crate find_folder;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
	(game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
	to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
	let gui_x = to_coord(x);
	let gui_y = to_coord(y);

	rectangle(
		color,
		[gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
		con.transform,
		g,
	);
}

pub fn draw_rectangle(
	color: Color,
	x: i32,
	y: i32,
	width: i32,
	height: i32,
	con: &Context,
	g: &mut G2d,
) {
	let x = to_coord(x);
	let y = to_coord(y);

	rectangle(
		color,
		[
			x,
			y,
			BLOCK_SIZE * (width as f64),
			BLOCK_SIZE * (height as f64),
		],
		con.transform,
		g,
	);
}

pub fn scoreboard(score: i32, glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
	let msg: &str = "Score: ";
	let together = &(format!("{}{}", msg, score));

	text(
		[1.0; 4],
		20,
		together,
		glyphs,
		con.transform.trans_pos([385.0, 19.0]),
		g,
	)
	.unwrap();
}

pub fn render_score(score: i32, glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
	let msg: &str = "Your Score: ";
	let together = &(format!("{}{}", msg, score));

	text(
		[1.0; 4],
		40,
		"GAME OVER",
		glyphs,
		con.transform.trans_pos([130.0; 2]),
		g,
	)
	.unwrap();
	text(
		[1.0; 4],
		30,
		together,
		glyphs,
		con.transform.trans_pos([150.0, 200.0]),
		g,
	)
	.unwrap();
	text(
		[1.0; 4],
		20,
		"Restarting in 3 seconds...",
		glyphs,
		con.transform.trans_pos([120.0, 300.0]),
		g,
	)
	.unwrap();
}
