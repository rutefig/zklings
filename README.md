# ZKlings 👽❤️

Welcome to ZKlings!
This project contains small exercises to get you familiar with Zero-Knowledge Proofs (ZKP) concepts, modular arithmetic and group theory and circuit implementation.
This includes working with various ZKP libraries and understanding the underlying mathematics!
Right now we support Rust (Halo2) and Circom.
This project is still in its early stages, so please be patient with us 🙏🏼 We have much more coming on our roadmap.

Please give your feedback as we are always looking to improve the quality of the exercises and the learning experience.

It is recommended to do the ZKlings exercises in parallel to reading resources on Zero-Knowledge Proofs and cryptography. Some recommended resources include:

- [Rare Skills ZK Book](https://www.rareskills.io/zk-book)
- [Learn 0xPARC](https://learn.0xparc.org/)

## Getting Started

### Installing Rust

Before installing ZKLings, you need to have _Rust installed_.
Visit [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for further instructions on installing Rust.
This will also install _Cargo_, Rust's package/project manager.

### Installing and Running ZKlings

For now you will need to clone the repo and run the following command in the root directory:

```bash
cargo run --bin zklings
```

Before running the command, make sure you have the latest Rust version by running `rustup update`. And if you want to run circom circuits, you need to have [circom](https://docs.circom.io/getting-started/installation/) installed on your machine.

### Still not ready

The following command will download and compile ZKlings:

```bash
cargo install zklings
```

If the installation fails…

Make sure you have the latest Rust version by running rustup update
Try adding the --locked flag: cargo install zklings --locked
Otherwise, please report the issue in our GitHub repository

Initialization
After installing ZKLings, run the following command to initialize the zklings/ directory:

```bash
zklings init
```

Now, go into the newly initialized directory and launch ZKLings for further instructions on getting started with the exercises:

```bash
cd zklings
zklings
```

Working environment
Editor
We recommend using VS Code with the rust-analyzer plugin.
Any editor that supports rust-analyzer should be sufficient for working on the exercises.
Doing exercises
The exercises are sorted by topic and can be found in the subdirectory exercises/<topic>.
For every topic, there is an additional README.md file with some resources to get you started on the topic.
We highly recommend that you have a look at them before you start 📚️
Exercises may include:

Rust code that needs to be completed or fixed
Circom circuits that need to be implemented
Mathematical problems related to ZKP concepts

Search for TODO and todo!() to find out what you need to change.
Ask for hints by entering h in the watch mode 💡
Watch Mode
After initialization, ZKLings can be launched by simply running the command zklings.
This will start the watch mode which walks you through the exercises in a predefined order.
It will rerun the current exercise automatically every time you change the exercise's file in the exercises/ directory.
Exercise List
In the watch mode (after launching zklings), you can enter l to open the interactive exercise list.
The list allows you to…

See the status of all exercises (done or pending)
c: Continue at another exercise (temporarily skip some exercises or go back to a previous one)
r: Reset status and file of an exercise (you need to reload/reopen its file in your editor afterwards)

See the footer of the list for all possible keys.
Continuing On
Once you've completed ZKLings, put your new knowledge to good use!
Continue practicing your ZKP skills by building your own projects, contributing to ZKLings, or exploring more advanced ZKP concepts and implementations.
Uninstalling ZKLings
If you want to remove ZKLings from your system, run the following command:

```bash
cargo uninstall zklings
```

Contributing
We welcome contributions! Please see our CONTRIBUTING.md file for details on how to contribute.
