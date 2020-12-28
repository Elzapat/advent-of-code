use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("8.txt").unwrap().trim().to_string();

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const LAYER_SIZE: usize = WIDTH * HEIGHT;

    let digits = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let layers = digits
        .chunks(LAYER_SIZE)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&layers));
    println!("Part 2: ");
    part_2(&layers, WIDTH, HEIGHT);
}

fn count_occ(layer: &[u32], d: u32) -> u32 {
    layer.iter().filter(|digit| **digit == d).count() as u32
}

fn part_1(layers: &Vec<&[u32]>) -> u32 {

    let layer_num = layers
        .iter()
        .enumerate()
        .map(|(i, layer)| (i, count_occ(layer, 0)))
        .min_by(|a, b| Ord::cmp(&a.1, &b.1))
        .unwrap()
        .0;

    count_occ(layers[layer_num], 1) * count_occ(layers[layer_num], 2)
}

fn part_2(layers: &Vec<&[u32]>, width: usize, height: usize) {

    let mut coords: (u32, u32) = (0, 0);
    let mut picture: HashMap<(u32, u32), u32> = HashMap::new();

    for layer in layers {
        for pixel in layer.iter() {
            if *pixel != 2 {
                picture.entry(coords).or_insert(*pixel); 
            }
            if coords.0 + 1 == width as u32 {
                coords.0 = 0;
                coords.1 += 1;
            } else {
                coords.0 += 1;
            }
        }
        coords = (0, 0);
    }

    for i in 0..height {
        for j in 0..width {
            match picture.get(&(j as u32, i as u32)) {
                Some(p) => match p {
                    0 => print!(" "),
                    1 => print!("#"),
                    _ => (),
                },
                None => (),
            }
        }
        println!("");
    }
}
