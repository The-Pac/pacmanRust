use std::ops::Add;
use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::{random, Rng, thread_rng};

const X_SIZE_MAP: usize = 30;
const Y_SIZE_MAP: usize = 12;
const NUMBER_GHOST: usize = 4;
const WALL_CHAR: char = '#';
const GHOST_FACE_CHAR: char = '👻';
const PACMAN_FACE_CHAR: char = '🟡';

fn create_map() -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![vec![WALL_CHAR; X_SIZE_MAP]; Y_SIZE_MAP];
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
                    *cell = WALL_CHAR;
                }

                if column_index < ((Y_SIZE_MAP - 1) / 2) + size_spawn_height && column_index > ((Y_SIZE_MAP - 1) / 2) {
                    *cell = ' '
                }
            }
            if cell_index == ((X_SIZE_MAP - 1) / 2) - (NUMBER_GHOST / 2) - 1 || cell_index == ((X_SIZE_MAP - 1) / 2) + (NUMBER_GHOST / 2) + 1 {
                if column_index <= ((Y_SIZE_MAP - 1) / 2) + size_spawn_height && column_index >= ((Y_SIZE_MAP - 1) / 2) {
                    *cell = WALL_CHAR;
                }
            }
        }
    }
}

fn spawn_pacman_and_ghosts(pacman: &Pacman, ghosts: &mut Vec<Ghost>, map: &mut Vec<Vec<char>>) {
    map[pacman.y][pacman.x] = pacman.face;
    for _i in 0..NUMBER_GHOST {
        let ghost = Ghost {
            is_hit: false,
            x: (X_SIZE_MAP / 2) - 1,
            y: (Y_SIZE_MAP / 2),
            face: GHOST_FACE_CHAR,
        };
        map[ghost.y][ghost.x] = ghost.face;
        ghosts.push(ghost)
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
    let mut ghosts: Vec<Ghost> = vec![];
    let mut map = create_map();
    let mut random_x: usize = thread_rng().gen_range(0..X_SIZE_MAP - 1);
    let mut random_y: usize = thread_rng().gen_range(0..Y_SIZE_MAP - 1);

    while map[random_y][random_y] == WALL_CHAR {
        random_x = thread_rng().gen_range(0..X_SIZE_MAP - 1);
        random_y = thread_rng().gen_range(0..Y_SIZE_MAP - 1);
    }
    let mut pacman: Pacman = Pacman {
        is_hit: false,
        x: random_x,
        y: random_y,
        face: PACMAN_FACE_CHAR,
    };


    spawn_pacman_and_ghosts(&pacman, &mut ghosts, &mut map);
    let is_game_done: bool = false;
    let device_state = DeviceState::new();
    let mut keys: Vec<Keycode>;


    while !is_game_done {
        keys = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::D => {
                    if map[pacman.y][pacman.x + 1] != WALL_CHAR {
                        map[pacman.y][pacman.x] = ' ';
                        pacman.x = pacman.x + 1;
                    }
                }
                Keycode::Q => {
                    if map[pacman.y][pacman.x - 1] != WALL_CHAR {
                        map[pacman.y][pacman.x] = ' ';
                        pacman.x = pacman.x - 1;
                    }
                }
                Keycode::S => {
                    if map[pacman.y + 1][pacman.x] != WALL_CHAR {
                        map[pacman.y][pacman.x] = ' ';
                        pacman.y = pacman.y + 1;
                    }
                }
                Keycode::Z => {
                    if map[pacman.y - 1][pacman.x] != WALL_CHAR {
                        map[pacman.y][pacman.x] = ' ';
                        pacman.y = pacman.y - 1;
                    }
                }
                _ => {}
            }
            map[pacman.y][pacman.x] = pacman.face;
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
