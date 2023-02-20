#![windows_subsystem = "windows"]

use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::window::PresentMode;

static WINDOW_WIDTH: f32 = 800.0;
static WINDOW_HEIGHT: f32 = 600.0;
static CELL_SPACE: f32 = 6.0;
static CELL_SIDE_NUM: i32 = 4;
static COLOR_BACKGROUND: Color = Color::rgb(187.0 / 255.0, 173.0 / 255.0, 160.0 / 255.0);
static COLOR_CELL_NULL: Color = Color::rgb(204.0 / 255.0, 192.0 / 255.0, 178.0 / 255.0);
static COLOR_CELL_2: Color = Color::rgb(238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0);
static COLOR_CELL_4: Color = Color::rgb(236.0 / 255.0, 224.0 / 255.0, 204.0 / 255.0);
static COLOR_CELL_8: Color = Color::rgb(242.0 / 255.0, 176.0 / 255.0, 120.0 / 255.0);
static COLOR_CELL_16: Color = Color::rgb(245.0 / 255.0, 148.0 / 255.0, 98.0 / 255.0);
static COLOR_CELL_32: Color = Color::rgb(245.0 / 255.0, 124.0 / 255.0, 95.0 / 255.0);
static COLOR_CELL_64: Color = Color::rgb(245.0 / 255.0, 92.0 / 255.0, 60.0 / 255.0);
static COLOR_CELL_128: Color = Color::rgb(235.0 / 255.0, 208.0 / 255.0, 113.0 / 255.0);
static COLOR_CELL_256: Color = Color::rgb(239.0 / 255.0, 203.0 / 255.0, 97.0 / 255.0);
static COLOR_CELL_512: Color = Color::rgb(239.0 / 255.0, 193.0 / 255.0, 45.0 / 255.0);
static COLOR_CELL_1024: Color = Color::rgb(239.0 / 255.0, 197.0 / 255.0, 63.0 / 255.0);
static COLOR_CELL_2048: Color = Color::rgb(238.0 / 255.0, 194.0 / 255.0, 46.0 / 255.0);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Bevy 2048".to_string(),
				position: WindowPosition::Centered,
				width: WINDOW_WIDTH,
				height: WINDOW_HEIGHT,
				present_mode: PresentMode::AutoNoVsync,
				resizable: false,
				..default()
			},
			..default()
		}))
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_startup_system(setup)
		.run();
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
	match cell_value {
		2 => COLOR_CELL_2,
		4 => COLOR_CELL_4,
		8 => COLOR_CELL_8,
		16 => COLOR_CELL_16,
		32 => COLOR_CELL_32,
		64 => COLOR_CELL_64,
		128 => COLOR_CELL_128,
		256 => COLOR_CELL_256,
		512 => COLOR_CELL_512,
		1024 => COLOR_CELL_1024,
		2048 => COLOR_CELL_2048,
		_ => COLOR_CELL_NULL
	}
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {
	commands.spawn(Camera2dBundle::default());

	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let mut y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into()).into(),
		material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
		transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
		..default()
	});

	for i in 0..CELL_SIDE_NUM {
		for j in 0..CELL_SIDE_NUM {
			commands.spawn(MaterialMesh2dBundle {
				mesh: meshes.add(shape::Box::new(side_length, side_length, 0.0).into()).into(),
				material: materials.add(ColorMaterial::from(COLOR_CELL_NULL)),
				transform: Transform::from_xyz(x_offset + (j as f32) * (side_length + CELL_SPACE), y_offset - (i as f32) * (side_length + CELL_SPACE), 0.0),
				..default()
			});
		}
	}

}











