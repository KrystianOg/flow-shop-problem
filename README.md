# fsp-gen

`fsp-gen` is a command-line tool designed to generate and solve instances of the Flow Shop Scheduling Problem (FSP). It supports multiple platforms, including Linux, macOS (both x86_64 and aarch64), and Windows.

## Features

- generate FSP instances with according to Taillard algorithm
- solve FSP instances using built-in heuristics or optimization algorithms
- support for various output formats
- cross-platform compatibility.

## Installation

### Download Precompiled Binaries

recompiled binaries are available for the following platforms:​

    Linux

    macOS (x86_64)

    macOS (aarch64)

    Windows

You can download the latest release from the [Releases](https://github.com/KrystianOg/flow-shop-problem/releases) page.

#### Linux

```bash
wget https://github.com/KrystianOg/flow-shop-problem/releases/download/v0.1.1/fsp-gen-v0.1.1-linux.tar.gz
tar -xzvf fsp-gen-v0.1.1-linux.tar.gz
sudo mv fsp-gen-v0.1.1-linux /usr/local/bin/fsp-gen
chmod +x /usr/local/bin/fsp-gen
```

#### macOS (x86_64)

```bash
wget https://github.com/KrystianOg/flow-shop-problem/releases/download/v0.1.1/fsp-gen-v0.1.1-macos-x86_64.tar.gz
tar -xzvf fsp-gen-v0.1.1-macos-x86_64.tar.gz
sudo mv fsp-gen-v0.1.1-macos-x86_64 /usr/local/bin/fsp-gen
chmod +x /usr/local/bin/fsp-gen
```

#### macOS (aarch64)

```bash
wget https://github.com/KrystianOg/flow-shop-problem/releases/download/v0.1.1/fsp-gen-v0.1.1-macos-aarch64.tar.gz
tar -xzvf fsp-gen-v0.1.1-macos-aarch64.tar.gz
sudo mv fsp-gen-v0.1.1-macos-aarch64 /usr/local/bin/fsp-gen
chmod +x /usr/local/bin/fsp-gen
```

#### Windows

1. Download [fsp-gen-v0.1.1-windows.zip](https://github.com/KrystianOg/flow-shop-problem/releases/download/v0.1.1/fsp-gen-v0.1.1-windows.zip)
2. Extract the archive.
3. Optionally, add the directory with `fsp-gen-v0.1.1-windows.exe` to your system `PATH`.

## Usage

After installation, you can use `fsp-gen` from your terminal or command prompt.

### Generate an FSP Instance

```bash
fsp-gen generate --jobs 5 --machines 3 --output instance.json
```

### Solve an FSP Instance

```bash
fsp-gen solve --input instance.json --algorithm greedy --output schedule.json
```

### Help

```bash
fsp-gen --help
```

## Checksums

You can verify the integrity of the downloaded files using the `SHA256SUMS.txt` file available with each release.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
