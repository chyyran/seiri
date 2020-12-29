# Building

*seiri* has a lot of moving parts, so building the app is a bit involved. Read through this document carefully and follow each step. If there's a missing step or the instructions don't work, file an issue.

## Installing Prerequisites

1. Rust

Install Rust at https://www.rust-lang.org, or through your package manager of choice.

1. CMake

CMake [3.12](https://cmake.org/) or higher is required to build `libkatatsuki-sys`.

3. Node

Building *seiri-neon* requires Node 14 LTS. Install Node at https://nodejs.org/en/ or through your package manager of choice. 

After, install the `neon` tool using `npm install -g neon-cli`.

## Building

Once you have installed all the prerequisites, and cloned the repository, you are now ready to build *seiri*.

1. Building *seiri-watcher*.
```bash
$ cd seiri-watcher
$ cargo build --release
```

If the build was successful, copy the resulting artifact `seiri-watcher`, or `seiri-watcher.exe` to the `seiri-client` folder.

2. Building *seiri-client-internals*
```bash
$ yarn install
$ yarn build
$ yarn run pack-asar
```
If the build was successful, copy the resulting artifact `ui.asar` to the `seiri-client` folder.

3. Building *seiri-client*
Before you build *seiri-client*, ensure that the *seiri-watcher* binary and the *ui.asar* file is present in the same folder as *package.json*.

```bash
$ yarn install
$ yarn build
$ yarn run dist
```

For more information on building the installation bundle, see https://www.electron.build/




