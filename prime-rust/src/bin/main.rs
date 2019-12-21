use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;
use prime_rust::pokermatch;
use std::time::SystemTime;
use std::process;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct LYMatch {
    alice: String,
    bob: String,
    result: i32
}

#[derive(Deserialize, Debug)]
// #[serde(transparent)]
struct LYMatches {
    matches: Vec<LYMatch>
}

fn read_match_from_file<P: AsRef<Path>>(path: P) -> Result<LYMatches, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    // println!("hello, world");
    let r:LYMatches = read_match_from_file("../alice_vs_bob/seven_cards_with_ghost.result.json").unwrap();
    let m:LYMatches = read_match_from_file("../alice_vs_bob/seven_cards_with_ghost.json").unwrap();
    let mut matches = Vec::new();
    for mm in &m.matches {
        let mut pm = pokermatch::LYMatch {
            alice: mm.alice.clone(),
            alice_score: 0,
            bob:   mm.bob.clone(),
            bob_score: 0,
            result: 0
        };
        matches.push(pm.clone());
    }

    let begin_at = SystemTime::now();
    for mm in matches.iter_mut() {
        pokermatch::process_match(mm);
    }

    let difference = SystemTime::now().duration_since(begin_at)
                    .expect("Clock may have gone backwards");
    let in_ms = difference.as_secs() * 1000 +
                    difference.subsec_nanos() as u64 / 1_000_000;
        
    println!("{:?} ms", in_ms);
    println!("hands:{}", m.matches.len());

    let mut i=0;
    for b in matches {
        // println!("result={}", b.result);
        if b.result != r.matches[i].result {
            println!("alice={},alice_score={},", b.alice, b.alice_score);
            println!("bob={},bob_score={},result={}", b.bob, b.bob_score, b.result);
            process::exit(0);
        }
        i+=1;
    }
}
