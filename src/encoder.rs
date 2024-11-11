use image::{DynamicImage, GenericImageView};
use std::io;

pub struct Encoder<'a> {
    img: DynamicImage,
    output_path: &'a str,
    message: &'a str,
}

impl<'a> Encoder<'a> {
    pub fn new(img_path: &'a str, output_path: &'a str, message: &'a str) -> Self {
        let img = image::open(img_path).expect("Error opening image");

        Encoder {
            img,
            output_path,
            message,
        }
    }

    pub fn encode(&self) -> io::Result<()> {
        let (width, height) = self.img.dimensions();
        let mut img_buffer = self.img.to_rgba8();

        let mut message_bits: Vec<u8> = self
            .message
            .bytes()
            .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1))
            .collect();

        message_bits.extend(vec![0; 8]);

        if message_bits.len() > (width * height * 4) as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Message is too long",
            ));
        }

        let mut bit_index = 0;
        for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
            for i in 0..4 {
                if bit_index < message_bits.len() {
                    pixel[i] = (pixel[i] & 0xFE) | message_bits[bit_index];
                    bit_index += 1;
                }
            }
        }

        img_buffer
            .save(self.output_path)
            .expect("Errore nel salvataggio dell'immagine.");

        Ok(())
    }
}
