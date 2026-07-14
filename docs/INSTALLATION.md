# Installation

> [!NOTE]
> Some packages or installers may supply an auto-updater that will ask to 
self-update when a new version of FlightCore is available.

FlightCore currently offers support for the folowing platforms:

* Windows (10, 11)
  * x86_64
* Linux
  * x86_64

It is important to note, it may be possible to compile the software for other
system architectures when building the app manually.

## Official software packages

### Windows Installer

Installation steps:

1. Download a `FlightCore_<version>_x64_en-US.msi` file from the 
[releases page](https://github.com/R2NorthstarTools/FlightCore/releases).
2. Run the downloaded file by double-clicking it.
3. Follow on-screen instructions.

### Linux AppImage

AppImage releases are currently borked.

## Community software packages

### Nixpkgs package

_This package is maintained by 
[username-generic](https://github.com/username-generic)._

Installation steps can be found 
[here](https://search.nixos.org/packages?channel=unstable&query=flightcore#show=flightcore).

## Building from source

Steps for building FlightCore from source can be found in 
[docs/DEVELOPMENT.md](DEVELOPMENT.md#setup).

### Creating a desktop entry

On Linux, to get FlightCore to show up within your application launchers, you
may also want to create an XDG Desktop Entry at
`/home/<username>/.local/share/applications/FlightCore.desktop` with the 
following contents:

```text
[Desktop Entry]
Categories=Game;PackageManager
Comment=Updater and mod manager for Northstar 
Exec=<path to local repository>/src-tauri/target/release/flightcore 
Icon=<path to local repository>/src-tauri/target/release/flightcore 
Name=FlightCore 
Terminal=false 
Type=Application 
Version=1.5
```

