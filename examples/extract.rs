use seven_zip::SevenZip;

fn main() {
    env_logger::init();

    let archive = SevenZip::new("example.7z", include_bytes!("example.7z"));
    let extracted = archive.extract();
    log::info!("Extracted {:?}", extracted.is_some());
    log::info!("Extracted {:?}", extracted.unwrap().len());
}
