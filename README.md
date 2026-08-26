# <img src="pages/public/img/ct.png" width="100" height="100"> Ct


Basic Text editor to encrypt fast and easy text.
It used the Galois/Counter Mode (GCM), an AEAD mode of operation for symmetric-key cryptographic. 
It's extremely secure and simple to use. Furthermore, it comes with a GUI less app ct_nox, so you can use it for your other programs.

## Usage

### GUI App
1. Run the app
2. Enter your text and a password
3. **Open CT File** — decrypt an encrypted file
4. **Save CT File** — encrypt and save to file
5. **Export Image** — encrypt text and encode as a 256x256 PNG with binary frame
6. **Import Image** — decode a binary frame PNG and decrypt the text

### CLI (ct_nox)

```bash
# Encrypt text
ct_nox encrypt -t "hello world" -p "mypassword"

# Decrypt text
ct_nox decrypt -t "encrypted_output" -p "mypassword"

# Encrypt file
ct_nox encrypt -f document.txt -p "mypassword" -o document.ct

# Decrypt file
ct_nox decrypt -f document.ct -p "mypassword"

# Encode as image (see IMAGE_STRIP.md)
ct_nox image-encode -t "secret" -p "mypassword" -o secret.png

# Decode from image
ct_nox image-decode -f secret.png -p "mypassword"
```

### Image Strip

Encode encrypted text as a binary frame around a PNG image. See [IMAGE_STRIP.md](IMAGE_STRIP.md) for full details.

```bash
ct_nox image-encode -f secret.txt -p "pass" -o secret.png
ct_nox image-decode -f secret.png -p "pass"
```

## Please Feedback
If you are using or are interested in this App, please send me some Feedback.
Any comment, request, critic or whatsoever is very welcome! <veto@myridia.com>

## Compile for Linux Debian
### Add lib requirements
```Bash
  sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
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


### Extra Repro
```
  git remote add codeberg ssh://git@codeberg.org/veto/ct
```

## Certificate on a Windows Box 
```
 wget https://github.com/myridia/ct/releases/download/main/Myridia.cer -OutFile Myridia.cer
 wget https://github.com/myridia/ct/releases/download/main/Ct.pfx -OutFile Ct.pfx
 certutil -addstore Root ".\Myridia.cer"
 certutil -p "passpass" -importpfx Ct.pfx
```
