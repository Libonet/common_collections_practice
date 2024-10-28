use common_collections_excercise::{median_mode, pig_latin, start_interface};

fn main() {
    let (median,mode) = median_mode(&[1,2,3,4,5,6,6,6,7,7]).unwrap();
    println!("First function: median={median} mode={mode}");

    println!("Second function: apple={}, first={}", pig_latin("apple").unwrap(), pig_latin("first").unwrap());

    println!("Third function: ");
    start_interface();
}
