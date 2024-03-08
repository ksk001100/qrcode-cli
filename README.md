QRCode CLI

# Install
```bash
$ git clone https://github.com/ksk001100/qrcode-cli
$ cd qrcode-cli
$ cargo install --path .
```

# Usage
```bash
$ qr encode "Hello, World!"
$ qr encode "Hello, World!" --output hello.png
$ qr e "Hello, World!" -o hello.png
$ qr decode hello.png
$ qr d hello.png
```