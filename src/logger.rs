use simplelog::*;
use std::fs::File;

pub fn init() {
    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        //.set_time_level(LevelFilter::Debug)
        .build();
    let _ = WriteLogger::init(
        LevelFilter::Trace, // logging all logs
        config,
        File::create("app.log").unwrap(),
    );
}
