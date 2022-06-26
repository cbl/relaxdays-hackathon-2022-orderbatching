
use std::fs;

use ob::{solution::{Solution, search}, model::{Model, Input}};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = args.get(1).expect("Missing input file name.");
    let output_file = args.get(2).expect("Missing output file name.");

    let input = fs::read_to_string(input_file).unwrap();

    let input = serde_json::from_str::<Input>(&input).unwrap();

    let model = Model::from(input.clone());

    let mut solution: Solution = Solution::from(model.clone());

    search(&mut solution, &model);
    
    let output = serde_json::to_string(&solution).unwrap();

    fs::write(output_file, output).expect("Unable to write file");

    // println!("articles = {:?}", model.orders.iter().map(|o|o.article_ids.len()).sum::<usize>());
    // println!("orders = {:?}", model.orders.len());

    // println!("solution articles = {:?}", solution.batches.iter().map(|b|b.items.len()).sum::<usize>());
    // println!("tour_cost() = {:?}", solution.tour_cost());
    // println!("rest_cost() = {:?}", solution.rest_cost());
    // println!("total_cost() = {:?}", solution.total_cost());

    // 807925
}
