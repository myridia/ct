# Image Strip — Binary Frame Encoding

Encode encrypted text into a 256x256 PNG image as a binary frame. Each character is represented by 8 pixels (black = 1, white = 0) arranged clockwise around the image perimeter. The ct logo sits in the center.

## How It Works

```
┌──────────────────────────────┐
│ ● ○ ● ● ○ ○ ● ○ ○ ● ● ○ ● │  ← bits go clockwise
│ ○                          ○ │
│ ●        [ct logo]         ● │
│ ○                          ○ │
│ ● ○ ○ ● ● ○ ● ○ ● ● ○ ○ ● │
└──────────────────────────────┘
```

- **Image size**: 256x256 pixels
- **Dot size**: 1x1 pixel
- **Path**: clockwise from top-left corner
- **Encoding**: black pixel = bit 1, white pixel = bit 0
- **8 pixels per character**, MSB first
- **Center**: ct64.png logo (64x64) with alpha blending
- **Max capacity**: 127 characters (1020 pixels / 8 bits)

## Data Format

The encrypted text follows the standard ct format:

```
{hex_nonce}/{hex_ciphertext}/{hex_mac}
```

Example: `a1b2c3d4e5f6/48656c6c6f/d7a8fbb3f7d2`

This string is then encoded as black/white pixels around the frame.

## CLI Usage (ct_nox)

### Encrypt text to image

```bash
ct_nox image-encode -t "my secret message" -p "password123" -o output.png
```

### Encrypt file to image

```bash
ct_nox image-encode -f secret.txt -p "password123" -o output.png
```

### Decode image to decrypted text

```bash
ct_nox image-decode -f output.png -p "password123"
```

### All CLI commands

```bash
# Text encrypt/decrypt
ct_nox encrypt -t "hello" -p "pass"
ct_nox decrypt -t "encrypted_text" -p "pass"

# File encrypt/decrypt
ct_nox encrypt -f input.txt -p "pass" -o output.ct
ct_nox decrypt -f input.ct -p "pass"

# Image encode/decode
ct_nox image-encode -t "secret" -p "pass" -o image.png
ct_nox image-encode -f file.txt -p "pass" -o image.png
ct_nox image-decode -f image.png -p "pass"
```

## GUI Usage

1. Type or paste your text
2. Enter a password
3. Click **Export Image** (toolbar) or **File → Export as Image**
4. Choose where to save the PNG
5. To decode: click **Import Image** (toolbar) or **File → Import from Image**
6. Select the PNG, enter the password, and the decrypted text appears

## Capacity

| Text length | Fits in 256x256? |
|-------------|-------------------|
| 1-127 chars | Yes |
| 128+ chars  | No (too long) |

The encrypted output includes overhead (nonce + MAC + separators), so the actual plaintext limit is shorter. A 50-character plaintext produces ~158 characters of ciphertext, which exceeds the 127-character limit. For longer texts, a larger image would be needed.

## Technical Details

- **Encryption**: AES-128-GCM (same as text mode)
- **PNG library**: `png` crate (Rust)
- **Logo**: embedded from `test/assets/icons/ct64.png` at compile time
- **Pixel order**: top row left-to-right, right column top-to-bottom, bottom row right-to-left, left column bottom-to-top
- **Alpha blending**: logo pixels are composited over white background

## Reference

Based on the C implementation in `/home/veto/webs/image/strip/` which encodes ASCII text as a horizontal binary strip in PNG.
