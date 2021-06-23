use rustlearn::datasets::iris;
use rustlearn::ensemble::random_forest::Hyperparameters;
use rustlearn::prelude::*;
use rustlearn::trees::decision_tree;

fn main() {
    let (data, target) = iris::load_data();

    let mut tree_params = decision_tree::Hyperparameters::new(data.cols());
    tree_params.min_samples_split(10).max_features(4);

    let mut model = Hyperparameters::new(tree_params, 10).one_vs_rest();

    model.fit(&data, &target).unwrap();

    let prediction = model.predict(&data).unwrap();
    println!("{:?}", data);
    println!("{:?}", target);
    println!("{:?}", prediction);
}
