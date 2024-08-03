# CT - CryptText
Basic Text editor to encrypt fast and easy text for Linux, Windows and Mac.
It used the Galois/Counter Mode (GCM), an AEAD mode of operation for symmetric-key cryptographic. It's extremely secure and simple to use. Furthermore, it comes with a GUI less app ct_nox, so you can use it for your other programs.

## Please Feedback
If you are using or are interested in this App, please send me some Feedback.
Any comment, request, critic or whatsoever is very welcome! <veto@myridia.com>

## Compile for Linux
### Add lib requirements
```Bash
  sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-de
```


## Compile to Windows
### Add requirements
```Bash
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```
### Build to Windows gnu
```Bash
cargo build --target x86_64-pc-windows-gnu --release 
```


## Compile to Mac
### Add requirements
```Bash

apt install \
    clang \
    gcc \
    g++ \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    cmake  \
    libxml2-dev 

rustup target add x86_64-apple-darwin
```
### Build to Mac
```Bash
PATH="$(pwd)/osxcross/target/bin:$PATH" cargo build --target x86_64-apple-darwin
```
