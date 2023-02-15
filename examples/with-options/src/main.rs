use plyr::options::*;
use plyr::Plyr;

fn main() {
    let controls = vec![
        "play-large".to_string(),
        "rewind".to_string(),
        "progress".to_string(),
        "pip".to_string(),
        "current-time".to_string(),
        "duration".to_string(),
    ];

    let youtube_options = YoutubeOptions::builder().start(20).end(60).build();

    let options = PlyrOptions::builder()
        .duration(100.0)
        .controls(controls)
        .youtube(youtube_options)
        .build();

    let _player = Plyr::new_with_options("#player", &options);
}
