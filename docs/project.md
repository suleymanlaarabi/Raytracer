# Project Architecture

This documentation details the architecture of the Raytracer project.

## Core Concepts

The Raytracer is split into a main binary and several dynamic libraries (plugins). The main logic loop resides in the `raytracer` package, handling:
- **Scene Parsing**: Reading `.ron` configurations.
- **Camera Setup**: Mathematical positioning, FOV, and rays generation.
- **Rendering Loop**: Intersecting rays with primitives, calculating lights and color from materials.
- **Output Management**: Pushing pixels to a `.ppm` file or rendering real-time into an SFML window.

## Plugin System

The raytracer utilizes dynamic loading to extend its functionalities. This ensures that the user can implement a new primitive or a new material without recompiling the main engine. Every plugin acts as a `.so` or `.dylib` library and implements standard traits expected by the core system.

### Types of Plugins:

1. **Primitives (`src/primitives/`)**: 
   - Define objects that can be mathematically intersected by a Ray.
   - Example: Sphere, Plane, Cone, Cylinder, Cube, OBJ.
2. **Lights (`src/lights/`)**: 
   - Define a light source, providing intensity, direction, or position.
   - Example: Point Light, Directional Light.
3. **Materials (`src/materials/`)**: 
   - Define the surface properties and how it reflects light/color.
   - Example: Flat Color.

## Math and Rendering

All linear algebra is contained inside the internal math module. The core loop generates rays starting from the camera origin, parsing through each virtual pixel in the viewport. If an intersection with a Primitive is found, the material parameters and interactions with Light instances dictate the final pixel color.
