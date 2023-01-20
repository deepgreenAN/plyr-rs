use plyr_rs::options::PlyrOpts;

fn main() {
    let opts = PlyrOpts::builder()
        .controls(vec!["play".to_string()])
        .build();
    println!("options: {:?}", opts);

    println!("json: {:?}", serde_json::to_string(&opts).unwrap());
}
