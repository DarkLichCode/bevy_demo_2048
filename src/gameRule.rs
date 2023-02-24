
use rand::Rng;
use crate::config::*;

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

pub fn Move_Value(direction: MOVE_DIRECTION, saveValue: &mut CELL_VALUE_SAVE) {
	match direction {
		MOVE_DIRECTION::NONE => return,
		_ => {
			saveValue.valueSave[0][0] += 1;
			saveValue.score += 1;
		}
	}
}