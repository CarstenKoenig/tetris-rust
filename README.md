# tetris-rust

very simple **Tetris** clone in **Rust** using the [*piston game-engine*](https://crates.io/crates/piston)

This is me learning a bit *Rust* - if you have any hints or just *WTF*s please help me learn ;)

## build/run

make sure you have **Rust** and **Cargo** installed (I guess it's recommended to use [Rustup](https://www.rust-lang.org/tools/install#:~:text=To%20install%20Rust%2C%20if%20you,follow%20the%20on%2Dscreen%20instructions.&text=If%20you%20are%20running%20Windows,follow%20the%20on%2Dscreen%20instructions.) which will get you both).

Then just clone this repo, and use *cargo* to build *and/or* run:

```sh
git clone https://github.com/CarstenKoenig/tetris-rust.git
cd tetris-rust
cargo build
cargo run
```

**Have Fun**

---

## Does this work everywhere?

[Piston](https://github.com/PistonDevelopers/piston/wiki/Frequently-Asked-Questions-(FAQ)) is cross-platform and I had no issue getting it to run under both Linux and Windows (I think I had to fetch some dependencies/dev-libraries for Linux but if you are on Linux this is probably no surprise for you - build it and google for the missing libraries+your distribution and you should find the solution easily).
