# Thorium

Thorium is the codename of a Rust-based cross-platform entity-component game engine and its associated scripting and design tools.

## Building

First, get and build the dependencies:
~~~
git submodule update --init --recursive
~~~

~~~
cd gl-rs
make
~~~

~~~
cd glfw-rs
make
~~~

Now, build the project itself:
~~~
rustc -L gl-rs/lib -L glfw-rs/lib main.rs
~~~
