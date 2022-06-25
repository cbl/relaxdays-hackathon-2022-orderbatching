
use std::fs;

use ob::{solution::{Solution, search}, model::{Model, Input}};

fn main() {
    let input = fs::read_to_string(
"examples/instance4.json"
    ).unwrap();

    let input = serde_json::from_str::<Input>(&input).unwrap();

    let model = Model::from(input);

    let mut solution: Solution = Solution::from(model.clone());

    search(&mut solution, &model);
    // let solution: Solution = serde_json::from_str(&output).unwrap();

    let output = serde_json::to_string_pretty(&solution).unwrap();
    // let output = serde_json::to_string(&solution).unwrap();

    fs::write("output/output.json", output).expect("Unable to write file");

    // println!("Hello, world!");
    // // println!("{:?}", solution);
    // println!("tour_cost() = {:?}", solution.tour_cost());
    // println!("rest_cost() = {:?}", solution.rest_cost());
    // println!("total_cost() = {:?}", solution.total_cost());

    // 807925
}
