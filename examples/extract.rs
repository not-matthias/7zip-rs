use seven_zip::SevenZip;

fn main() {
    env_logger::init();

    let mut archive = SevenZip::new(include_bytes!("../example.7z"));
    let extracted = archive.extract();
    log::info!("Extracted {:?}", extracted);
}
