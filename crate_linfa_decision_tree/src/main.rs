use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};

fn main() {
    // Initialize the default set of parameters
    let params = DecisionTree::params();
    // Set the parameters to the desired values
    let params = params
        .split_quality(SplitQuality::Entropy)
        .max_depth(Some(5))
        .min_weight_leaf(2.);

    // Load the data
    let (train, val) = linfa_datasets::iris().split_with_ratio(0.9);
    // Fit the decision tree on the training data
    let tree = params.fit(&train).unwrap();
    // Predict on validation and check accuracy
    let val_accuracy = tree
        .predict(&val)
        .confusion_matrix(&val)
        .unwrap()
        .accuracy();
    assert!(val_accuracy > 0.99);
}
