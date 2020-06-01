# Rust Workshop

This repository consists of all the rust workshops that I used to learn Rust and to get a good grasp over it.

## Getting Started

Installing Rust should hopefully be relatively straight forward, but if you run into
any issues, please do let me know so I can help troubleshoot.

To install the toolchain, first go to [rustup.rs](https://rustup.rs), and follow the
instructions. This should install the compiler, Cargo (Rust's build tool), and
other things like documentation.

You can check that this all worked by:

- opening your terminal (PowerShell, cmd.exe, bash, etc. should all work)
- entering `cargo new hello-world`
- changing into the newly created `hello-world` directory
- running `cargo run`
- if you see "Hello, world!" printed to your screen, then everything should be good!

I recommend VS Code as the programming environment for Rust though there are relatively
sizable portions of the community that use vim, emacs, Sublime, and IntelliJ. Using
Rust in Visual Studio is possible, but it might be a bit buggy.

If you use VS Code, make sure to install the Rust extension and open your hello-world
project in VS Code. You should then be prompted to install the Rust language server (RLS).
Once the RLS is installed, your environment should be ready.
