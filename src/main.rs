use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::window::PresentMode;


static WINDOW_WIDTH: f32 = 800.0;
static WINDOW_HEIGHT: f32 = 600.0;
static CELL_SPACE: f32 = 6.0;
static CELL_SIDE_NUM: i32 = 4;
static COLOR_BACKGROUND: Color = Color::rgb(187.0 / 255.0, 173.0 / 255.0, 160.0 / 255.0);
static COLOR_CELL_NULL: Color = Color::rgb(204.0 / 255.0, 192.0 / 255.0, 178.0 / 255.0);

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

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {
	commands.spawn(Camera2dBundle::default());

	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let mut y_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
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
				transform: Transform::from_xyz(x_offset + (j as f32) * (side_length + CELL_SPACE), y_offset + (i as f32) * (side_length + CELL_SPACE), 0.0),
				..default()
			});
		}
	}
	//
	// commands.spawn(MaterialMesh2dBundle {
	// 	mesh: meshes.add(shape::Box::new(50.0, 50.0, 0.0).into()).into(),
	// 	material: materials.add(ColorMaterial::from(Color::BLUE)),
	// 	transform: Transform::from_xyz(50.0, 0.0, 0.0),
	// 	..default()
	// });
	//
	// commands.spawn(MaterialMesh2dBundle {
	// 	mesh: meshes.add(shape::Box::new(50.0, 50.0, 0.0).into()).into(),
	// 	material: materials.add(ColorMaterial::from(Color::BLACK)),
	// 	transform: Transform::from_xyz(0.0, 0.0, 0.0),
	// 	..default()
	// });
}











