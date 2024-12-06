#[derive(Debug, Clone, Copy, Default)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn main() {
    let input = aoc_base::read_input();
    let mut ingredients = vec![];

    for line in input.lines() {
        let properties = line.split(':').skip(1).next().unwrap();
        let mut ingredient = Ingredient::default();

        for prop in properties.split(',') {
            let mut parts = prop.split(' ').skip(1);

            let name = parts.next().unwrap().trim();
            let amount = parts.next().unwrap().trim().parse::<i64>().unwrap();

            match name {
                "capacity" => ingredient.capacity = amount,
                "durability" => ingredient.durability = amount,
                "flavor" => ingredient.flavor = amount,
                "texture" => ingredient.texture = amount,
                "calories" => ingredient.calories = amount,
                _ => unreachable!(),
            }
        }

        ingredients.push(ingredient);
    }

    let mut max_score_p1 = 0;
    let mut max_score_p2 = 0;

    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                for l in 0..=100 {
                    if i + j + k + l != 100 {
                        continue;
                    }

                    let score_p1 = compute_score(&ingredients, &[i, j, k, l], false);
                    if score_p1 > max_score_p1 {
                        max_score_p1 = score_p1;
                    }

                    let score_p2 = compute_score(&ingredients, &[i, j, k, l], true);
                    if score_p2 > max_score_p2 {
                        max_score_p2 = score_p2;
                    }
                }
            }
        }
    }

    println!("Part 1: {max_score_p1}");
    println!("Part 2: {max_score_p2}");
}

fn compute_score(ingredients: &[Ingredient], teaspoons: &[i64], count_calories: bool) -> i64 {
    let mut result = 1;
    let mut calories = 0;

    for i in 0..4 {
        let mut prop_score = ingredients
            .iter()
            .enumerate()
            .map(|(j, ingredient)| {
                teaspoons[j]
                    * match i {
                        0 => ingredient.capacity,
                        1 => ingredient.durability,
                        2 => ingredient.flavor,
                        3 => ingredient.texture,
                        _ => unreachable!(),
                    }
            })
            .sum::<i64>();

        calories += teaspoons[i] * ingredients[i].calories;

        if prop_score < 0 {
            prop_score = 0;
        }

        result *= prop_score;
    }

    if count_calories && calories != 500 {
        return 0;
    }

    result
}
