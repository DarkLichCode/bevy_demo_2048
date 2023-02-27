
use rand::Rng;
use crate::config::*;

// 初始化，空白面板，在随机的两个位置生成 2
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

// 判断游戏胜负
pub fn check_result(saveValue: &mut CELL_VALUE_SAVE) -> VICTORY_or_DEFEAT {
	// 有2048判断玩家胜利
	for i in 0..saveValue.valueSave.len() {
		for j in 0..saveValue.valueSave[i].len() {
			if saveValue.valueSave[i][j] == 2048 {
				return VICTORY_or_DEFEAT::VICTORY;
			}
		}
	}

	// 未胜利，有空位，游戏继续
	for i in 0..saveValue.valueSave.len() {
		for j in 0..saveValue.valueSave[i].len() {
			if saveValue.valueSave[i][j] == 0 {
				return VICTORY_or_DEFEAT::NONE;
			}
		}
	}

	// 没有空位，但是有可合并的点，游戏继续
	for i in 0..saveValue.valueSave.len()-1 {
		for j in 0..saveValue.valueSave[i].len()-1 {
			if saveValue.valueSave[i][j] == saveValue.valueSave[i + 1][j] ||
				saveValue.valueSave[i][j] == saveValue.valueSave[i][j + 1] {
				return VICTORY_or_DEFEAT::NONE;
			}
		}
	}

	// 以上都不满足，无法再移动，玩家输
	return VICTORY_or_DEFEAT::DEFEAT;
}

// 移动函数
pub fn Move_Value(direction: MOVE_DIRECTION, saveValue: &mut CELL_VALUE_SAVE) {
	// 判断是否要新生成 2或4 的flag
	let mut generateNew = false;
	match direction {
		MOVE_DIRECTION::NONE => return ,
		MOVE_DIRECTION::RIGHT => generateNew = To_Right(&mut saveValue.valueSave),
		_ => {
			return;
		}
	}

	// 在空位生成新的数
	if generateNew {
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