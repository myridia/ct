source $HOME/.cargo/env
cargo watch --no-gitignore -w 'src/main.rs'  \
      -w 'src' \
      -w 'Cargo.toml' \
      -x run


      



