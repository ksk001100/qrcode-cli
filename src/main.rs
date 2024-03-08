use image::Luma;
use qrcode::QrCode;
use seahorse::{App, Command, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [args]", env!("CARGO_PKG_NAME")))
        .action(|c| c.help())
        .command(encode_command())
        .command(decode_command());

    app.run(args);
}

fn encode_command() -> Command {
    Command::new("encode")
        .description("Encode a string")
        .usage("qr encode [args]")
        .alias("e")
        .flag(
            Flag::new("output", FlagType::String)
                .alias("o")
                .description("Output filename"),
        )
        .action(|c| {
            let text = c.args[0].clone();
            let filename = c.string_flag("output").unwrap_or("qr.png".to_string());
            let code = QrCode::new(text).unwrap();
            let image = code.render::<Luma<u8>>().build();
            match image.save(&filename) {
                Ok(_) => println!("QR code saved to {}", &filename),
                Err(e) => println!("Error saving QR code: {}", e),
            }
        })
}

fn decode_command() -> Command {
    Command::new("decode")
        .description("Decode a QR code")
        .usage("qr decode [args]")
        .alias("d")
        .action(|c| {
            let filename = c.args[0].clone();
            let img = match image::open(filename) {
                Ok(img) => img,
                Err(e) => {
                    println!("Error opening image: {}", e);
                    std::process::exit(1);
                }
            };
            let decoder = bardecoder::default_decoder();
            let results = decoder.decode(&img);
            for result in results {
                println!("{}", result.unwrap());
            }
        })
}
