use image::DynamicImage;

pub struct Decoder {
    img: DynamicImage,
}

impl Decoder {
    pub fn new(img_path: &str) -> Self {
        let img = image::open(img_path).expect("Errore nell'apertura dell'immagine.");

        Decoder { img }
    }

    pub fn decode(&self) -> String {
        let img_buffer = self.img.to_rgba8();

        let mut message_bits: Vec<u8> = Vec::new();

        // Decode message from image
        for (_, _, pixel) in img_buffer.enumerate_pixels() {
            for i in 0..4 {
                message_bits.push(pixel[i] & 1);
            }
        }

        let mut message_bytes: Vec<u8> = Vec::new();
        
        // Convert bits to bytes
        for bits in message_bits.chunks(8) {
            let byte = bits.iter().fold(0, |acc, &bit| (acc << 1) | bit);
            if byte == 0 {
                break;
            }
            message_bytes.push(byte);
        }

        String::from_utf8_lossy(&message_bytes).into_owned()
    }
}
