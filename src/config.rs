
use bevy::prelude::*;
use rand::Rng;

pub static WINDOW_WIDTH: f32 = 800.0;
pub static WINDOW_HEIGHT: f32 = 600.0;
pub static CELL_SPACE: f32 = 6.0;
pub static CELL_SIDE_NUM: u32 = 4;
pub static COLOR_BACKGROUND: Color = Color::rgb(187.0 / 255.0, 173.0 / 255.0, 160.0 / 255.0);
pub static COLOR_CELL_NULL: Color = Color::rgb(204.0 / 255.0, 192.0 / 255.0, 178.0 / 255.0);
pub static COLOR_CELL_2: Color = Color::rgb(238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0);
pub static COLOR_CELL_4: Color = Color::rgb(236.0 / 255.0, 224.0 / 255.0, 204.0 / 255.0);
pub static COLOR_CELL_8: Color = Color::rgb(242.0 / 255.0, 176.0 / 255.0, 120.0 / 255.0);
pub static COLOR_CELL_16: Color = Color::rgb(245.0 / 255.0, 148.0 / 255.0, 98.0 / 255.0);
pub static COLOR_CELL_32: Color = Color::rgb(245.0 / 255.0, 124.0 / 255.0, 95.0 / 255.0);
pub static COLOR_CELL_64: Color = Color::rgb(245.0 / 255.0, 92.0 / 255.0, 60.0 / 255.0);
pub static COLOR_CELL_128: Color = Color::rgb(235.0 / 255.0, 208.0 / 255.0, 113.0 / 255.0);
pub static COLOR_CELL_256: Color = Color::rgb(239.0 / 255.0, 203.0 / 255.0, 97.0 / 255.0);
pub static COLOR_CELL_512: Color = Color::rgb(239.0 / 255.0, 193.0 / 255.0, 45.0 / 255.0);
pub static COLOR_CELL_1024: Color = Color::rgb(239.0 / 255.0, 197.0 / 255.0, 63.0 / 255.0);
pub static COLOR_CELL_2048: Color = Color::rgb(238.0 / 255.0, 194.0 / 255.0, 46.0 / 255.0);

pub static COLOR_BROWN: Color = Color::rgb(120.0 / 255.0, 110.0 / 255.0, 100.0 / 255.0);
pub static COLOR_WHITE: Color = Color::rgb(245.0 / 255.0, 250.0 / 255.0, 240.0 / 255.0);

pub enum MOVE_DIRECTION {
	NONE,
	UP,
	DOWN,
	LEFT,
	RIGHT
}

#[derive(Resource)]
pub struct CELL_VALUE_SAVE {
	pub(crate) valueSave: Vec<Vec<u32>>
}

#[derive(Component)]
pub struct CELL_VALUE;

// 初始化
pub fn Init_cell_value_save() -> Vec<Vec<u32>> {
	let mut cell_value_save_temp: Vec<Vec<u32>> = Vec::new();
	let mut pos_save: Vec<Vec<usize>> = Vec::new();
	for i in 0..CELL_SIDE_NUM {
		let mut cell_value_save_temp_row: Vec<u32> = Vec::new();
		for j in 0..CELL_SIDE_NUM {
			cell_value_save_temp_row.push(0);
			let temp_pos = vec![i as usize, j as usize];
			pos_save.push(temp_pos);
		}
		cell_value_save_temp.push(cell_value_save_temp_row);
	}

	let mut index = rand::thread_rng().gen_range(0..16) as usize;
	cell_value_save_temp[pos_save[index][0]][pos_save[index][1]] = 2;
	pos_save.remove(index);
	index = rand::thread_rng().gen_range(0..15) as usize;
	cell_value_save_temp[pos_save[index][0]][pos_save[index][1]] = 2;
	return cell_value_save_temp;
}

pub fn Move_Value(direction: MOVE_DIRECTION, saveValue: &mut Vec<Vec<u32>>) {
	match direction {
		MOVE_DIRECTION::NONE => return,
		_ => {
			saveValue[0][0] += 1;
		}
	}
}