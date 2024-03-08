use image::Luma;
use qrcode::QrCode;
use seahorse::{App, Command};
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
        .action(|c| {
            let text = c.args[0].clone();
            let filename = match c.args.get(1) {
                Some(filename) => filename,
                None => "qrcode.png",
            };
            let code = QrCode::new(text).unwrap();
            let image = code.render::<Luma<u8>>().build();
            image.save(filename).unwrap();
        })
}

fn decode_command() -> Command {
    Command::new("decode")
        .description("Decode a QR code")
        .usage("qr decode [args]")
        .alias("d")
        .action(|c| {
            let filename = c.args[0].clone();
            let img = image::open(filename).unwrap();
            let decoder = bardecoder::default_decoder();
            let results = decoder.decode(&img);
            for result in results {
                println!("{}", result.unwrap());
            }
        })
}
