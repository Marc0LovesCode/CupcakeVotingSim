use cupcake::default;
use cupcake::traits::{KeyGeneration, PKEncryption, SKEncryption, AdditiveHomomorphicScheme};
use rand::Rng;

fn main() {
    println!("CLI Voting System for the next Hokage using Cupcake Homomorphic Encryption");

    let scheme = default();
    let (pk, sk) = scheme.generate_keypair();

    const BYTES_PER_COUNT: usize = 4; 

    let mut vote_for_luffy = scheme.encrypt(&vec![0; BYTES_PER_COUNT], &pk);
    let mut vote_for_naruto = scheme.encrypt(&vec![0; BYTES_PER_COUNT], &pk);

    // Generate votes for 1000 users
    for _ in 0..1000 {
        let vote = if rand::thread_rng().gen_bool(0.5) {
            // Vote for Luffy
            scheme.encrypt(&vec![1, 0, 0, 0], &pk)
        } else {
            // Vote for Naruto
            scheme.encrypt(&vec![1, 0, 0, 0], &pk)
        };

        if rand::thread_rng().gen_bool(0.5) {
            scheme.add_inplace(&mut vote_for_luffy, &vote);
        } else {
            scheme.add_inplace(&mut vote_for_naruto, &vote);
        }
    }

    println!("Tallying votes...");

    let votes_for_luffy: Vec<u8> = scheme.decrypt(&vote_for_luffy, &sk);
    let votes_for_naruto: Vec<u8> = scheme.decrypt(&vote_for_naruto, &sk);

    let total_luffy = u32::from_le_bytes(votes_for_luffy[0..BYTES_PER_COUNT].try_into().unwrap());
    let total_naruto = u32::from_le_bytes(votes_for_naruto[0..BYTES_PER_COUNT].try_into().unwrap());

    println!("Final tally:");
    println!("Votes for Luffy: {}", total_luffy);
    println!("Votes for Naruto: {}", total_naruto);

    if total_luffy > total_naruto {
        println!("Luffy is the winner! Congratulations!");
    } else if total_luffy < total_naruto {
        println!("Naruto is the winner! Congratulations!");
    } else {
        println!("It's a tie between Luffy and Naruto!");
    }
}
