
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

pub fn check_result(saveValue: &mut CELL_VALUE_SAVE) -> VICTORY_or_DEFEAT {
	for i in 0..saveValue.valueSave.len() {
		for j in 0..saveValue.valueSave[i].len() {
			if saveValue.valueSave[i][j] == 2048 {
				return VICTORY_or_DEFEAT::VICTORY;
			}
		}
	}

	for i in 0..saveValue.valueSave.len() {
		for j in 0..saveValue.valueSave[i].len() {
			if saveValue.valueSave[i][j] == 0 {
				return VICTORY_or_DEFEAT::NONE;
			}
		}
	}

	for i in 0..saveValue.valueSave.len()-1 {
		for j in 0..saveValue.valueSave[i].len()-1 {
			if saveValue.valueSave[i][j] == saveValue.valueSave[i + 1][j] ||
				saveValue.valueSave[i][j] == saveValue.valueSave[i][j + 1] {
				return VICTORY_or_DEFEAT::NONE;
			}
		}
	}

	return VICTORY_or_DEFEAT::DEFEAT;
}

pub fn Move_Value(direction: MOVE_DIRECTION, saveValue: &mut CELL_VALUE_SAVE) {
	let mut generateTwo = false;
	match direction {
		MOVE_DIRECTION::NONE => return ,
		MOVE_DIRECTION::RIGHT => generateTwo = To_Right(&mut saveValue.valueSave),
		_ => {
			return;
		}
	}

	if generateTwo {
		let mut temp: u32 = rand::thread_rng().gen_range(0..10) as u32;
		if temp > 0 {
			temp = 2;
		} else {
			temp = 4;
		}
		let mut pos_save: Vec<Vec<usize>> = Vec::new();
		for i in 0..saveValue.valueSave.len() {
			for j in 0..saveValue.valueSave[i].len() {
				if saveValue.valueSave[i][j] == 0 {
					let pos = vec![i, j];
					pos_save.push(pos);
				}
			}
		}
		let index = rand::thread_rng().gen_range(0..pos_save.len());
		saveValue.valueSave[pos_save[index][0]][pos_save[index][1]] = temp;
	}

	return ;
}

// 向右移动
pub fn To_Right(saveValue: &mut Vec<Vec<u32>>) -> bool {
	for i in 0..saveValue.len() {
		for j in 0..saveValue[i].len() {
			if saveValue[i][j] == 0 {
				return true;
			}
		}
	}

	return false;
}