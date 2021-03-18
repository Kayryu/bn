extern crate bn;
extern crate rand;
use bn::{Group, Fr, G1, G2, pairing, multi};

fn pedersen() {
    let rng = &mut rand::thread_rng();

    // input a = 2, b = 3, c = 5, to prove : f is a + b = c.
    let a = Fr::from_str("2").unwrap();
    let b = Fr::from_str("3").unwrap();

    let c = Fr::from_str("5").unwrap();
    assert!(c == (a+b));

    let alpha = Fr::random(rng);
    let beta = Fr::random(rng);

    let g = G1::one();
    let h = G1::one() * 100.into();

    // g^a + h ^ alpha
    let c_a = g * a + h * alpha;
    // g^b + h ^ beta
    let c_b = g * b + h * beta;

    // g^(a+b) + h ^(alpha + beta)
    let c_c = g * c + h * (alpha + beta);
    println!("{:?}", c_c);

    assert_eq!(c_c, c_a + c_b);
}

fn main() {
    pedersen();
}
