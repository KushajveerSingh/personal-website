---
title: A Gentle Introduction To Rust
nav_order: 2
parent: Rust
---

<!-- prettier-ignore-start -->
# A Gentle Introduction To Rust
{: .no_toc }

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

<!-- prettier-ignore-end -->

## Useful commands

-   `rustup update` - Upgrade rust.
-   `rustup doc` - View offline docs.
-   `cargo init` - Create rust project in current directory.
-   Makefile to compile and run in single command

    ```makefile
    run:
    	rustc $(FILE) -o out.exe
    	./out.exe
    ```

    Run from terminal using `make run FILE=00-basics.rs`.

-   `.pdb` files created on Windows to store debug info.
-   `target` folder created when running `cargo run`.
-   `.rs.bk` backup files created by _rustfmt_.
-   Add to `.gitignore`
    ```
    debug/
    target/
    *.pdb
    **/*.rs.bk
    ```
