```bash
javac Gcm.java
jar cvfe Gcm.jar Gcm Gcm.class
java -jar Gcm.jar
```

With no arguments the program runs a self-test. It also works as a small CLI:

```bash
# encrypt -> prints the v2 ciphertext
java Gcm encrypt "password" "hello world"

# decrypt -> prints the plaintext
java Gcm decrypt "password" "v2/pbkdf2-sha256/..."

# without the jar
java Gcm encrypt "password" "hello world"
```

## Format

```
v2/pbkdf2-sha256/{iterations}/{hex_salt}/{hex_nonce}/{hex_ciphertext}/{hex_mac}
```

- Password bytes: UTF-8
- KDF: PBKDF2-HMAC-SHA256, 600000 iterations, 16-byte salt, 16-byte key
- Cipher: AES-128-GCM, 12-byte nonce, 16-byte tag
- Hex output is lowercase; the parser accepts either case.
- The old `{hex_nonce}/{hex_ciphertext}/{hex_mac}` format is still decrypted.

The Rust implementation in `ct_nox` writes and reads the same format.

## Rust <-> Java test

```bash
# Rust encrypt -> Java decrypt
CT=$(ct_nox encrypt -t "hello world" -p "password")
java Gcm decrypt "password" "$CT"

# Java encrypt -> Rust decrypt
CT=$(java Gcm encrypt "password" "hello world")
ct_nox decrypt -t "$CT" -p "password"
```

Both directions should print `hello world`.
