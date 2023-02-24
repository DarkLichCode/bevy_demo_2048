#![windows_subsystem = "windows"]

mod config;
use config::*;

use rand::Rng;

use bevy::asset::{Asset, HandleId};
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private::de::Content::String;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::text::Text2dBounds;
use bevy::window::PresentMode;

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
		.add_system(keyboard_input)
		.run();
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
	match cell_value {
		2 => COLOR_CELL_2.clone(),
		4 => COLOR_CELL_4.clone(),
		8 => COLOR_CELL_8.clone(),
		16 => COLOR_CELL_16.clone(),
		32 => COLOR_CELL_32.clone(),
		64 => COLOR_CELL_64.clone(),
		128 => COLOR_CELL_128.clone(),
		256 => COLOR_CELL_256.clone(),
		512 => COLOR_CELL_512.clone(),
		1024 => COLOR_CELL_1024.clone(),
		2048 => COLOR_CELL_2048.clone(),
		_ => COLOR_CELL_NULL.clone()
	}
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {

	// 初始化存储数组
	let mut cell_value_save_temp: Vec<Vec<u32>> = Init_cell_value_save();
	let mut cell_background_save: Vec<HandleId> = Vec::new();
	// 计算左上方格偏移
	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let mut y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;


	commands.spawn(Camera2dBundle::default());

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into()).into(),
		material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
		transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
		..default()
	});

	// 初始化文字信息
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: side_length / 2.0,
		color: COLOR_BROWN,
	};
	let box_size = Vec2::new(side_length, side_length);

	for i in 0..CELL_SIDE_NUM {
		for j in 0..CELL_SIDE_NUM {

			// 格中显示内容
			let mut text = "";
			if cell_value_save_temp[i as usize][j as usize] == 2 {
				text = "2";
			}

			let materialColor = materials.add(ColorMaterial::from(cell_color(cell_value_save_temp[i as usize][j as usize])));
			cell_background_save.push(materialColor.id());
			// 绑定格，根据数字来确定格的颜色
			commands.spawn(
				MaterialMesh2dBundle {
					mesh: meshes.add(shape::Box::new(side_length, side_length, 0.0).into()).into(),
					material: materialColor,
					transform: Transform::from_xyz(
						x_offset + (j as f32) * (side_length + CELL_SPACE),
						y_offset - (i as f32) * (side_length + CELL_SPACE),
						0.0),
					..default()
				}
			);

			// 绑定数字
			commands.spawn((
				Text2dBundle {
					text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
					text_2d_bounds: Text2dBounds {
						// Wrap text in the rectangle
						size: box_size,
					},
					transform: Transform::from_xyz(
						x_offset + (j as f32) * (side_length + CELL_SPACE),
						y_offset - (i as f32) * (side_length + CELL_SPACE),
						1.0),
					..default()
				},
				CELL_VALUE
			));
		}
	}

	// 将存储数组设为资源
	commands.insert_resource(
		CELL_VALUE_SAVE{
			valueSave: cell_value_save_temp.clone(),
			cellBackGround: cell_background_save,
			score: 0
		}
	);

	commands.spawn(
		Text2dBundle {
			text: Text::from_sections(
				[
					TextSection::new("SCORE\n", text_style.clone()),
					TextSection::new("0", text_style.clone()),
				]

			),
			text_2d_bounds: Text2dBounds {
				// Wrap text in the rectangle
				size: box_size,
			},
			transform: Transform::from_xyz(
				-WINDOW_WIDTH / 2.0,
				WINDOW_HEIGHT / 2.0,
				0.0,
			),
			..default()
		}
	);
}

fn keyboard_input(
	keyboard_input: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	mut cell_Value_Save: ResMut<CELL_VALUE_SAVE>,
	mut text_query: Query<(&mut Text), (With<CELL_VALUE>)>,
	mut score_query: Query<(&mut Text), (Without<CELL_VALUE>)>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {
	let mut moved = MOVE_DIRECTION::NONE;
	if keyboard_input.just_pressed(KeyCode::Up) {
		moved = MOVE_DIRECTION::UP;
	}
	if keyboard_input.just_pressed(KeyCode::Down) {
		moved = MOVE_DIRECTION::DOWN;
	}
	if keyboard_input.just_pressed(KeyCode::Right) {
		moved = MOVE_DIRECTION::RIGHT;
	}
	if keyboard_input.just_pressed(KeyCode::Left) {
		moved = MOVE_DIRECTION::LEFT;
	}

	match moved {
		MOVE_DIRECTION::NONE => return,
		_ => {
			let mut i = 0;
			Move_Value(moved, &mut cell_Value_Save);

			score_query.single_mut().sections[1].value = cell_Value_Save.score.to_string();

			let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
			let font = asset_server.load("fonts/FiraSans-Bold.ttf");
			let mut text_style = TextStyle {
				font,
				font_size: side_length / 2.0,
				color: COLOR_BROWN,
			};

			for mut text in text_query.iter_mut() {
				let cell_value_temp = cell_Value_Save.valueSave[i / 4][i % 4];

				if cell_value_temp > 4 {
					text_style.color = COLOR_WHITE;
				} else {
					text_style.color = COLOR_BROWN;
				}

				if cell_value_temp != 0 {
					text.sections[0].style = text_style.clone();
					text.sections[0].value = cell_Value_Save.valueSave[i / 4][i % 4].to_string();
				} else {
					text.sections[0].value = "".to_string();
				}
				materials.set_untracked(cell_Value_Save.cellBackGround[i], ColorMaterial::from(cell_color(cell_Value_Save.valueSave[i / 4][i % 4])));
				i += 1;
			}

		}
	}

}









