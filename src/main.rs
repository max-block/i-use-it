use i_use_it::{generate_readme, read_data};

fn main() {
    let data = read_data();
    // println!("{:#?}", data);
    generate_readme(data);
}