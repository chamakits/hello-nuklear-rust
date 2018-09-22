# Hello nuklear rust

Just me playing with nuklear and rust.

Started with [nuklear-test](https://github.com/snuk182/nuklear-test), and deleted and changed as much as I could without it crashing.

Also in the process I'm trying to put big messy things in methods, mostly based on how I would do things in Java, no idea if it makes sense to do it like that in Rust.

## Setup

I'm currently running this on windows. No idea right now if it works elsewhere, but hopefully? I think so, because the example was based on OpenGL. Probably nees some minor changes.

Also, to be able to use it, I HAD to use gcc/mingw. Specifically the 64 bit variant.

Install mingw64 [from sourceforge here](https://sourceforge.net/projects/mingw-w64/). Make sure in the install to choose 64 bit architecture, as the installer defaults to 32 bit.

Then put the bin directory on Path, and should be good to go.

Then 'cargo run' does the trick.