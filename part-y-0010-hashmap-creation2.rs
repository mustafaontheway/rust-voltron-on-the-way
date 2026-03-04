use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    score_by_id(&mut scores,4296, 77);
    score_by_id(&mut scores, 3345, 89);
    score_by_id(&mut scores, 5578, 65);

    println!("{:?}", scores) // {5578: 65, 4296: 77, 3345: 89}
}

fn score_by_id(scores: &mut HashMap<u16, u8>, id: u16, score: u8) {

    scores.insert(id, score);
}



// cargo run main.rs
