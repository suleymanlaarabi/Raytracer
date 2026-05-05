# Raytracer

A modular and extensible Raytracer written in Rust. This project features dynamic plugin loading for primitives, materials, and lights, enabling easy extension without recompiling the core engine. It supports outputting renders to `.ppm` image files or displaying them directly in an SFML window.

## Features

- **Modular Architecture**: Uses a workspace with dynamic libraries (`.so` / `.dylib`) for primitives, lights, and materials.
- **Primitives**: Sphere, Plane, Cylinder, Cone, Cube, and OBJ file support.
- **Lights**: Point Light, Directional Light.
- **Materials**: Flat Color.
- **Configuration Engine**: Uses RON (Rusty Object Notation) parsing for scene descriptions.
- **SFML Integration**: Option to render directly into a window.
- **Hot-reloading/Watch mode**: Integrated filesystem watcher to re-render the image automatically when the scene file is modified.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Cargo)
- [SFML](https://www.sfml-dev.org/) (for the graphical window support)
- GNU Make

## Building the Project

Use the provided `Makefile` to compile the core raytracer and all its plugins in release mode:

```bash
make all
```
This command builds the project and copies the compiled dynamic libraries to the `plugins/` folder to ensure the engine can find them at runtime.

## Running the Raytracer

You can run the raytracer to render the default configuration (`config.ron`) and display it using the SFML window:

```bash
make run
```
Which is roughly equivalent to running:
```bash
cargo run --release -- ./config.ron --sfml
```

### Watch Mode

If you are editing a scene and want to see the changes update automatically, you can use the watch mode:

```bash
make watch
```
*Note: This utilizes `inotifywait` (Linux) to watch for file changes.*

## Scene Configuration

Scenes are described using RON (Rusty Object Notation). Here are a few examples included in the workspace:
- `config.ron`
- `car.ron`, `city.ron`, `house.ron` (located in `./assets/` or repository root)

## Documentation

This project uses [Doxygen](https://www.doxygen.nl/) for generating code documentation. 

To generate the documentation, run:
```bash
doxygen Doxyfile
```

## Cleaning up

To remove the compiled target directories:
```bash
make clean
# or to fully wipe out the target folder:
make fclean
```
