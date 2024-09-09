use simplelog::*;
use std::fs::File;

pub fn init() {
    // log
    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        //.set_time_level(LevelFilter::Debug)
        .build();
    let _ = WriteLogger::init(
        LevelFilter::Trace, // すべてのレベルのログを記録
        config,
        File::create("app.log").unwrap(),
    );
}
