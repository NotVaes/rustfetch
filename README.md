# RustFetch

A fast system information tool written in Rust, inspired by neofetch.

![RustFetch Demo] [image](https://github.com/user-attachments/assets/7bbbc34f-5ba6-4cc2-969a-c5844f4b463d)


## Features

- 🚀 **Fast** - Written in Rust for maximum performance
- 🖥️ **Cross-platform** - Works on Windows, Linux, and macOS
- 🎨 **Colorful output** - Beautiful ANSI colored display
- 📊 **Comprehensive info** - Shows CPU, GPU, memory, disk, and more
- 🔋 **Battery status** - Displays battery information on laptops
- 🌐 **Network info** - Shows local IP address

## Installation

### From crates.io (Recommended)

```bash
cargo install rustfetch
```

### From GitHub

```bash
cargo install --git https://github.com/NotVaes/rustfetch
```

### From Source

```bash
git clone https://github.com/NotVaes/rustfetch
cd rustfetch
cargo install --path .
```

## Usage

Simply run:

```bash
rustfetch
```

## System Information Displayed

- **OS**: Operating system and architecture
- **Host**: System manufacturer and model
- **Kernel**: Kernel version
- **Uptime**: System uptime
- **Packages**: Installed packages (Chocolatey/Winget on Windows)
- **Shell**: Current shell and version
- **Display**: Screen resolution and refresh rate
- **DE/WM**: Desktop environment and window manager
- **Theme**: Current system theme
- **Font**: System font information
- **CPU**: Processor information with core count
- **GPU**: Graphics card information
- **Memory**: RAM usage and total
- **Swap**: Swap/page file usage
- **Disk**: Storage usage for all drives
- **Network**: Local IP address
- **Battery**: Battery status and percentage
- **Locale**: System locale

## Platform Support

- ✅ **Windows** - Full support with PowerShell integration
- ✅ **Linux** - Full support with /proc filesystem
- 🚧 **macOS** - Basic support (contributions welcome!)

## Building from Source

Requirements:
- Rust 1.70 or later
- Cargo

```bash
git clone https://github.com/YOUR_USERNAME/rustfetch
cd rustfetch
cargo build --release
```

The binary will be available at `target/release/rustfetch`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development

```bash
git clone https://github.com/YOUR_USERNAME/rustfetch
cd rustfetch
cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [neofetch](https://github.com/dylanaraps/neofetch) and FastFetch
- Built with ❤️ in Rust
