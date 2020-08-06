# Ram Downloader
**Made in GTK and Rust.**

Ram Downloader is a simple GUI utility that helps download more RAM. You can select
how much RAM you want to download and what CloudRAM protocol to use. 

Ram Downloader supports
all operating systems, including Microsoft Windows 7 and later, 
Apple macOS 10.7 and higher as well as UNIX and Linux.

## Screenshots
![Linux Screenshot](https://raw.githubusercontent.com/daniel071/images-for-readme/master/coolScreenshot.png)
![Windows Screenshot](https://raw.githubusercontent.com/daniel071/images-for-readme/master/ramDownloaderWindows.png)

## Installation
Download the binary for your operating system from [here](https://github.com/daniel071/gtkRamDownloader/releases/) and execute it. 
**If you are on Windows or macOS, you must** [**install GTK**](https://www.gtk.org/docs/installations/) **first.** 
Currently there are only Windows and Linux binaries. macOS binaries are coming soon. If you are on macOS, you can follow the compiling instructions.

## Compiling
Make sure you have [git](https://git-scm.com/) for your operating system.
Make sure you also have the [rust compiler](https://www.rust-lang.org/tools/install) and the [GTK](https://www.gtk.org/docs/installations/) framework.

1. Download the source code of the repo with `git clone https://github.com/daniel071/gtkRamDownloader.git`
2. CD into the new directory
3. Run `cargo run` if you want to test it or `cargo build` if you want to create an executable.
