use aoc_tools::input::input::read_lines;

fn create_recipes(nbr_recipes: i64) -> i64 {
    let mut recipes: Vec<i64> = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipes.len() < nbr_recipes as usize + 10 {
        // Create new recipes
        let comb = recipes[elf1] + recipes[elf2];
        if comb > 9 {
            let new_recipe2 = comb % 10;
            let new_recipe1 = (comb - new_recipe2) / 10;
            recipes.push(new_recipe1);
            recipes.push(new_recipe2);
        } else {
            recipes.push(comb);
        }
        // Find the new "current" recipe for each elf
        elf1 = (elf1 + 1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2] as usize) % recipes.len();
    }
    let p1 = recipes[(nbr_recipes as usize)..(nbr_recipes as usize)+10]
        .iter().enumerate()
        .map(|(i, x)| x * 10i64.pow(9-i as u32))
        .sum();
    p1
}

fn create_recipes2(recipe_nbr: i64) -> i64 {
    let num_len = recipe_nbr.to_string().len();
    let mut recipes: Vec<i64> = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut curr_num = -1i64;
    while curr_num != recipe_nbr {
        // Create new recipes
        let comb = recipes[elf1] + recipes[elf2];
        if comb > 9 {
            let new_recipe2 = comb % 10;
            let new_recipe1 = (comb - new_recipe2) / 10;
            recipes.push(new_recipe1);
            if recipes.len() > num_len && recipes[(recipes.len() as usize)-num_len..(recipes.len() as usize)]
            .iter().enumerate()
            .map(|(i, x)| x * 10i64.pow((num_len-1-i) as u32))
            .sum::<i64>() == recipe_nbr {
                return (recipes.len() - num_len) as i64;
            }
            recipes.push(new_recipe2);
        } else {
            recipes.push(comb);
        }
        // Find the new "current" recipe for each elf
        elf1 = (elf1 + 1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2] as usize) % recipes.len();
        if recipes.len() > num_len {
            curr_num = recipes[(recipes.len() as usize)-num_len..(recipes.len() as usize)]
            .iter().enumerate()
            .map(|(i, x)| x * 10i64.pow((num_len-1-i) as u32))
            .sum();
        }
    }
    (recipes.len() - num_len) as i64
}

fn main() {
    let recipe_nbr = read_lines::<i64>("input.txt").unwrap()[0];
    println!("part1: {}", create_recipes(recipe_nbr));
    println!("part2: {}", create_recipes2(recipe_nbr));
}
