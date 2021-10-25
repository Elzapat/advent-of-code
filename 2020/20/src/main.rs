// Advent of code day 20
// I spent quite some time on this one but couldn't find a way to make it work
// I ended up watching a video to understand the problem and a solution
// So this solution is strongly similar to this one here: https://www.youtube.com/watch?v=Px_nqwalSKs&
// All credit to him!

use std::{ fmt, fs, collections::HashSet };

type Id = u64;

const TILE_SIZE: usize = 10;

#[derive(Default, Debug, Clone)]
struct Tile {
    pub id: Id,
    pub image: [String; TILE_SIZE],
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.image.iter() {
            write!(f, "{}\n", line).unwrap()
        }

        Ok(())
    }
}

impl Tile {
    pub fn rotate_right(&mut self) {
        unimplemented!();
    }

    pub fn rotate_left(&mut self) {
        let tile_copy = self.clone();

        for i in 0..TILE_SIZE {
            for j in 0..self.image[i].len() {
                self
                    .image[TILE_SIZE - j - 1]
                    .replace_range(
                        i..i + 1,
                        &tile_copy.image[i][j..j + 1],
                    );
            }
        }
    }

    pub fn flip(&mut self) {
        for line in self.image.iter_mut() {
            *line = line.chars().rev().collect::<String>();
        }
    }

    pub fn down_side(&self) -> String {
        self.image[TILE_SIZE - 1].clone()
    }

    pub fn up_side(&self) -> String {
        self.image[0].clone()
    }

    pub fn left_side(&self) -> String {
        self
            .image
            .iter()
            .map(|line| line.chars().nth(0).unwrap())
            .collect::<String>()
    }

    pub fn right_side(&self) -> String {
        self
            .image
            .iter()
            .map(|line| line.chars().last().unwrap())
            .collect::<String>()
    }
}

fn main() {
    let file = fs::read_to_string("20_ex.txt").unwrap();
    let lines = file.split('\n').collect::<Vec<&str>>();
    let tiles = parse_tiles(lines);
    let (grid_size, tiles) = complete_tiles(tiles);
    let mut visited_tiles = HashSet::<Id>::new();
    let mut grid = vec![Vec::new(); grid_size];
    for i in 0..grid.len() {
        grid[i] = vec![Tile::default(); grid_size];
    }

    search(grid_size, 0, 0, &tiles, &mut visited_tiles, &mut grid);
}

fn search(grid_size: usize, row: usize, col: usize, tiles: &Vec<Tile>, visited_tiles: &mut HashSet<Id>, grid: &mut Vec<Vec<Tile>>) {
    if row == grid_size - 1 {
        println!("Part 1: {}", grid[0][0].id * grid[grid_size - 1][0].id * grid[0][grid_size - 1].id * grid[grid_size - 1][grid_size - 1].id);
        std::process::exit(0);
    }

    for tile in tiles {
        if !visited_tiles.contains(&tile.id) {
            if row > 0 && tiles[grid_size * (row - 1) + col].down_side() != tile.up_side() {
                continue;
            }

            if col > 0 && tiles[grid_size * row + (col - 1)].right_side() != tile.left_side() {
                continue;
            }

            grid[row][col] = tile.clone();
            visited_tiles.insert(tile.id);

            if col == grid_size - 1 {
                search(grid_size, row + 1, 0, tiles, visited_tiles, grid);
            } else {
                search(grid_size, row, col + 1, tiles, visited_tiles, grid);
            }

            visited_tiles.remove(&tile.id);
        }
    }
}

fn complete_tiles(tiles: Vec<Tile>) -> (usize, Vec<Tile>) {
    let mut complete_tiles = Vec::new();

    for mut tile in tiles.into_iter() {
        for _f in 0..2 {
            for _r in 0..4 {
                complete_tiles.push(tile.clone());
                tile.rotate_left();
            }
            tile.flip();
        }
    }

    ((((complete_tiles.len() / 8) as f32).sqrt()).round() as usize, complete_tiles)
}

fn parse_tiles(raw_tiles: Vec<&str>) -> Vec<Tile> {
    let mut progress = 0;
    let mut tiles = Vec::new();

    'main: loop {
        let mut tile = Tile::default();

        for i in 0..12 {
            let line = if let Some(l) = raw_tiles.get(progress) {
                l.to_string()
            } else {
                break 'main
            };

            match i {
                0 => tile.id = line[5..9].parse::<Id>().unwrap(),
                1..=10 => tile.image[i - 1] = line,
                _ => {},
            }

            progress += 1;
        }

        tiles.push(tile);
    }

    tiles
}
