// iter - This borrows each element of the collection through each iteration.
// Thus leaving the collection untouched and available for reuse after the loop.
//
// into_iter - This consumes the collection so that on each iteration the exact data is provided.
// Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
//
// iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    let mo_names = vec!["Alice", "Jim", "Ferris"];

    for name in mo_names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("mo_names: {:?}", mo_names);
    // ^ Comment out this line

    let mut the_names = vec!["Roger", "Tom", "Ferris"];

    for name in the_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("the_names: {:?}", the_names);
}
