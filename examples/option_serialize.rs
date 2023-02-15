use plyr::PlyrOptions;

fn main() {
    let options = PlyrOptions::builder().duration(100.0).build();
    println!("{}", serde_json::to_string(&options).unwrap());
}
