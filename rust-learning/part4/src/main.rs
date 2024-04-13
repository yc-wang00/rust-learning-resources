fn main() {
    let mut city_names = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    let last_city = match city_names.pop() {
        Some(inner_value) => {inner_value}
        None => {""}
    };

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    city_names.push("Rustville");

    // 👉 TODO now that we've done that, use `.push()` to put last_city
    //    back in `city_names`.

    println!("Here is the full list of cities:");
    // 👉 TODO print each of the city names.
    //
    // 💡 TIP: Here's an example of `for` loop syntax:
    //
    for my_element in city_names.iter() { 
        println!("{:?}", my_element)
    }
}
