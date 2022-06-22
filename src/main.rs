use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};

const X_SIZE_MAP: usize = 30;
const Y_SIZE_MAP: usize = 12;
const NUMBER_GHOST: usize = 4;

fn init() {
    let mut ghosts: Vec<Ghost> = vec![];
    let pacman: Pacman = Pacman {
        is_hit: false,
        x: 0,
        y: 0,
        face: '🟡',
    };
    let map = create_map();
    spawn_pacman_and_ghosts(pacman, ghosts, map)
}

fn create_map() -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![vec!['#'; X_SIZE_MAP]; Y_SIZE_MAP];
    create_map_pattern(&mut map);
    return map;
}

fn create_map_pattern(map: &mut Vec<Vec<char>>) {
    let mut size_spawn_height: usize = 2;
    if NUMBER_GHOST > 4 {
        size_spawn_height = NUMBER_GHOST / 2;
    }


    for (column_index, column) in map.iter_mut().enumerate() {
        for (cell_index, cell) in column.iter_mut().enumerate() {
            if column_index != 0 && cell_index != 0 && column_index != Y_SIZE_MAP - 1 && cell_index != X_SIZE_MAP - 1 {
                *cell = ' ';
            }

            if cell_index >= ((X_SIZE_MAP - 1) / 2) - (NUMBER_GHOST / 2) && cell_index <= ((X_SIZE_MAP - 1) / 2) + (NUMBER_GHOST / 2) {
                if column_index == ((Y_SIZE_MAP - 1) / 2) && cell_index != ((X_SIZE_MAP - 1) / 2)
                    || column_index == ((Y_SIZE_MAP - 1) / 2) + size_spawn_height {
                    *cell = '#';
                }

                if column_index < ((Y_SIZE_MAP - 1) / 2) + size_spawn_height && column_index > ((Y_SIZE_MAP - 1) / 2) {
                    *cell = ' '
                }
            }
            if cell_index == ((X_SIZE_MAP - 1) / 2) - (NUMBER_GHOST / 2) - 1 || cell_index == ((X_SIZE_MAP - 1) / 2) + (NUMBER_GHOST / 2) + 1 {
                if column_index <= ((Y_SIZE_MAP - 1) / 2) + size_spawn_height && column_index >= ((Y_SIZE_MAP - 1) / 2) {
                    *cell = '#';
                }
            }
        }
    }
}

fn spawn_pacman_and_ghosts(pacman: Pacman, mut ghosts: Vec<Ghost>, mut map: Vec<Vec<char>>) {
    *map[*pacman.x][*pacman.y] = *pacman.face;
    for i in 0..NUMBER_GHOST {
        let ghost = Ghost {
            is_hit: false,
            x: 0,
            y: 0,
            face: '👻',
        };
        &map[&ghost.y][&ghost.x] = &ghost.face;
        &ghosts.push(ghost)
    }
}

pub struct Ghost {
    is_hit: bool,
    x: usize,
    y: usize,
    face: char,
}

pub struct Pacman {
    is_hit: bool,
    x: usize,
    y: usize,
    face: char,
}

fn main() {
    let map = create_map();
    let is_game_done: bool = false;
    let device_state = DeviceState::new();
    let mut keys: Vec<Keycode>;


    while !is_game_done {
        keys = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::D => {}
                Keycode::Q => {}
                Keycode::S => {}
                Keycode::Z => {}
                _ => {}
            }
        }
        for column in &map {
            for cell in column {
                print!("{}", cell);
            }
            println!();
        }
        //clear screen
        print!("{}[2J", 27 as char);

        thread::sleep(Duration::from_millis(100))
    }
}
