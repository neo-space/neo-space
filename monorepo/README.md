... this directory is for practice with wasm and rendering

generating a new rust lib (--lib creates code used by other code like wasm)
```bash
cargo new --lib name_your_project
```

generating a new wasm project from a template
```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template --name whatever-you-want
```

The project has a `Cargo.toml` in its base. You have to add the path to the new project you are making here to the `members` section for it to be recognized and become runnable. 

go into any of the libs in this directory and read their `README.md` to run them.