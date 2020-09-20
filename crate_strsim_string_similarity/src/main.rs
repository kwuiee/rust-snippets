extern crate strsim;

use strsim::{
    damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance, sorensen_dice,
};

fn main() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why),
    }

    assert_eq!(levenshtein("kitten", "sitting"), 3);

    assert!((normalized_levenshtein("kitten", "sitting") - 0.571).abs() < 0.001);

    assert_eq!(osa_distance("ac", "cba"), 3);

    assert_eq!(damerau_levenshtein("ac", "cba"), 2);

    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.272).abs() < 0.001);

    assert!((jaro("Friedrich Nietzsche", "Jean-Paul Sartre") - 0.392).abs() < 0.001);

    assert!((jaro_winkler("cheeseburger", "cheese fries") - 0.911).abs() < 0.001);

    assert_eq!(
        sorensen_dice("web applications", "applications of the web"),
        0.7878787878787878
    );
}
