mod encoder;
mod secret;

fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("Need an argument to encode.");

    println!("Encoded data:");
    println!("{}", encoder::aes_encode(arg));
}
