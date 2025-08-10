# JS tokenizer

a highly unoptimized / inefficient attempt at tokenizing JavaScript with a proof-of-concept syntax-highlighter that uses this module.

# Building / Running standalone

Simply clone this repository and use `cargo` to run or build the project. 

Ensure `git` and `cargo` are installed. Then, run the following commands:

```sh
git clone https://github.com/mud-ali/js-tokenizer.git
cd js-tokenizer 

# modify the sample js in data/main.js, then
cargo run # to run once
cargo build --release # compile a production executable
```

Create a file in `data/main.js` and run the executable.

```sh
mkdir data
echo "var b = 12;" > data/main.js
./tokenizer
```

# Using as a crate

To use this as a crate, you can reference the library from its git URL. More information on this can be found in [the cargo documentation](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).