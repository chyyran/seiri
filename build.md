# Building

*seiri* has a lot of moving parts, so building the app is a bit involved. Read through this document carefully and follow each step. If there's a missing step or the instructions don't work, file an issue.

## Installing Prerequisites

1. Rust
Install Rust (1.25.0) or higher at https://www.rust-lang.org, or through your package manager of choice. *seiri* requires the nightly compiler, so once rustup is installed, run `rustup default nightly`.

2. .NET Core
Building *libkatatsuki-sys* requires .NET Core SDK 2.1.300 Preview 2 at a minimum to be installed. Follow the instructions on your operating system of choice at https://www.microsoft.com/net/download/dotnet-core/sdk-2.1.300-preview2.

3. CoreRT
*libkatatsuki-sys* relies on the experimental technology [CoreRT](https://github.com/dotnet/corert). There are prerequisites to building CoreRT projects, specified at the https://github.com/dotnet/corert/blob/master/Documentation/prerequisites-for-building.md. At a minimum, CMake 3.8.0 is required, and the Visual C++ MSVC Toolchain on Windows. On Linux, clang-3.9 is a requirement, higher versions are not supported.

4. Node
Building *seiri-neon* requires Node 8.11.1 LTS. Install Node at https://nodejs.org/en/ or through your package manager of choice. 

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
$ npm install
$ npm build
$ npm pack-asar
```
If the build was successful, copy the resulting artifact `ui.asar` to the `seiri-client` folder.

3. Building *seiri-client*
Before you build *seiri-client*, ensure that the *seiri-watcher* binary and the *ui.asar* file is present in the same folder as *package.json*.

```bash
$ npm install
$ npm build
$ npm dist
```

For more information on building the installation bundle, see https://www.electron.build/




