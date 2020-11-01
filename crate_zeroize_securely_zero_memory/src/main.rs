use zeroize::Zeroize;

fn main() {
    // Protip: don't embed secrets in your source code.
    // This is just an example.
    let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
    // [ ... ] open the air shield here

    // Now that we're done using the secret, zero it out.
    secret.zeroize();
    println!("{:?}", secret);
}
