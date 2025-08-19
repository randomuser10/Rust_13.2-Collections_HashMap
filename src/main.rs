use::std::collections::HashMap;
fn main() {
    // creating a mutable collection using HashMap
    let mut scores = HashMap::new();

    //Inserting the keys and value to the collection

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    
    //logging the values to the console
    println!("{:?}", scores);

    //adding a value if not present
    scores.entry(String::from("Purple")).or_insert(80);
    scores.entry(String::from("Yellow")).or_insert(90);
    println!("{scores:?}");

    //outputing the values in loop
    for (keys, values) in &scores{
        println!("{keys}: {values}");
    };

    // Overwriting a Value

    scores.insert(String::from("Green"), 30);
    scores.insert(String::from("Green"), 45);
    println!("{scores:?}");

    let field_name = String::from("Fav color");
    let field_value = String::from("Blue");

    let mut new_value = HashMap::new();
    new_value.insert(field_name, field_value);
    println!("{new_value:?}");

    // Updating a Value Based on the Old Value
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    };
    println!("{map:?}");

}
