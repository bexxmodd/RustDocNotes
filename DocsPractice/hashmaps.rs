use std::collections::HashMap;

fn main() {

    // stores data on the heap
    let mut scores = HashMap::new();

    scores.insert(String::from("Beka"), 12);
    scores.insert(String::from("Ana"), 13);
    scores.insert(String::from("Nerse"), 19);

    // NOTE: create hashmap using collect()
    let teams = vec![
        String::from("AC Milan"),
        String::from("Juventus"),
        String::from("SSC Napoli")
    ];

    let table_scores = vec![66, 62,60];

    let mut table: HashMap<_, _> =
        teams.into_iter().zip(table_scores.into_iter()).collect();

    let mut i = 1;
    for (key, val) in &table {
        println!("{}. {}\t{}", i, key, val);
        i += 1;
    }

    let winner = table.get(&String::from("Lazio"));
    match winner {
        Some(val) => println!("Winner is {:?}", val),
        None => println!("We Have no winners :("),
    }

    //Update the scores for a team (Overwriting a Value)
    table.insert(String::from("Juventus"), 63);

    // only inserting a value if the key has no valkue
    table.entry(String::from("AC Milan")).or_insert(69); // will not make any change
    table.entry(String::from("SS Lazio")).or_insert(59);

    for (key, val) in &table {
        println!("{}\t{}", key, val);
    }

    for (_, val) in table.iter_mut() {
        *val += 2;
    }

    for (key, val) in &table {
        println!("{}\t{}", key, val);
    }

}