use seq_macro::seq;

seq!(N in 64..=127 {
    #[derive(Debug)]
    enum Demo {
        // Expands to Variant64, Variant65, ...
        #(
            Variant~N,
        )*
    }
});

fn main() {
    let tuple = (1000, 100, 10);
    let mut sum = 0;

    // Expands to:
    //
    //     sum += tuple.0;
    //     sum += tuple.1;
    //     sum += tuple.2;
    //
    // This cannot be written using an ordinary for-loop because elements of
    // a tuple can only be accessed by their integer literal index, not by a
    // variable.
    seq!(N in 0..=2 {
        sum += tuple.N;
    });

    assert_eq!(sum, 1110);

    assert_eq!("Variant99", format!("{:?}", Demo::Variant99));
}
