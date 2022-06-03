use std::{io::{Cursor, BufWriter}, fs::File};

use image::DynamicImage;

const URL: &'static str = "https://thispersondoesnotexist.com/image";

fn get_single_image_response(client: &reqwest::blocking::Client) -> anyhow::Result<DynamicImage> {
    let response = client.execute(
        client.get(URL).build().unwrap()
    )?;

    image::io::Reader::new(Cursor::new(response.bytes()?))
        .with_guessed_format()?.decode().map_err(|a| a.into())
}

fn get_multiple_image_responses(n: u32) {
    let client = reqwest::blocking::Client::new();
    for i in 0..n {
        let dynamic_image = get_single_image_response(&client).unwrap();
        let file_dir = format!("output/{}.jpeg", i);
        let file = File::create(file_dir).unwrap();
        dynamic_image.write_to(&mut BufWriter::new(file), image::ImageOutputFormat::Jpeg(255)).unwrap();
    }
}

fn main() {
    get_multiple_image_responses(100);
}
