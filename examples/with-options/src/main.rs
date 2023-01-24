use plyr::options::PlyrOptions;
use plyr::Plyr;

fn main() {
    let _player = Plyr::new_with_options("#player", &PlyrOptions::builder().duration(50.0).build());
}
