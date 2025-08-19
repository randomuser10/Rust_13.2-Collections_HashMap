use::std::collections::HashMap;
fn main() {
    // creating a mutable collection using HashMap
    let mut scores = HashMap::new();

    //Inserting the keys and value to the collection

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    
    //logging the values to the console
    println!("{:?}", scores);

    //outputing the values in loop
    for (keys, values) in &scores{
        println!("{keys}: {values}");
    };
}
