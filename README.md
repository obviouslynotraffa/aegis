# Aegis

Aegis is a CLI tool written in Rust that enables encryption and decryption of text from an image, using least significant bit [Steganography](https://en.wikipedia.org/wiki/Steganography). 

In this technique, each byte of the secret message is embedded into the image by replacing the least significant bit of the pixel values with the bits of the message. In this case, the LSB of the red, green, blue and alpha channels in pixel is used to hide four bits of the message. As a result, the image looks visually unchanged, but it can contain hidden information that can be extracted later.

## Showcase
Before            |  After
:-------------------------:|:-------------------------:
![](imgs/original_image.png)  |  ![](imgs/modified_image.png)

## Usage
### Encoding
To encode a message in an image, run the following command:
```
cargo run -- -e <image_path> <output_image_path> <message>
```
- `<image_path>`: the path to the image where the message will be hidden
- `<output_image_path>`: the path where the encoded image will be saved
- `<message>`: the message you want to hide

### Decoding
To decode a hidden message from an image, run the following command:
```
cargo run -- -d <image_path>
```
- `<image_path>`: the path to the image that contains the hidden message