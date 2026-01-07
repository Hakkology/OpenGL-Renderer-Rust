# OpenGL Renderer in Rust 🦀

A high-performance, modular 3D rendering engine built with **Rust** and **OpenGL 3.3+**. This project demonstrates advanced graphics programming concepts including dynamic lighting, shadow mapping, model loading, and a custom UI system, all architected with Rust's safety and performance in mind.

![OpenGL Renderer](https://via.placeholder.com/1280x720?text=OpenGL+Renderer+Screenshot)

## ✨ Features

### 🎨 Rendering & Graphics
*   **Modern OpenGL (Core Profile):** Uses strict OpenGL 3.3+ core profile functions via the `gl` crate.
*   **Phong Lighting Model:** 
    *   **Directional Light:** Simulates sun-like global lighting.
    *   **Point Lights:** Multiple colored point lights with attenuation (distance fade).
    *   **Material System:** Supports colored and textured materials with configurable ambient, diffuse, and specular properties.
*   **Advanced Shadows:**
    *   **Directional Shadows:** High-res shadow mapping for the main light source.
    *   **Omnidirectional Shadows:** Point light shadows using Cubemaps and Geometry Shaders.
    *   **PCF (Percentage-Closer Filtering):** Soft shadow edges for a realistic look.
*   **Skybox:** High-quality cubemap skybox background.
*   **Texture Mapping:** Support for diffuse maps, repeating textures, and UV scaling.

### 🏗️ Engine Architecture
*   **Scene Graph System:** Hierarchical object management with `SceneObject3D`.
*   **Component-Based Logic:** Flexible `Controller` trait for adding behaviors (e.g., `OrbitController`, `RotationController`, `FloatingController`).
*   **Asset Management:** Centralized `AssetManager` for caching and loading shaders, textures, and models.
*   **Model Loading:** Robust model importer using `russimp` (Assimp bindings) supporting **OBJ, FBX**, and other formats with automatic triangulation and UV flipping.
*   **Generic Primitive Shapes:** Built-in generators for Cube, Sphere, Plane, Capsule, and quad primitives.

### 🖥️ User Interface (UI)
*   **Custom 2D Renderer:** Orthographic projection based UI rendering integrated into the 3D pipeline.
*   **Text Rendering:** High-quality text rendering using `rusttype`.
*   **Interactive Widgets:** 
    *   **Inspector Panel:** Real-time modification of object transforms (Position, Rotation, Scale).
    *   **Buttons:** Interactive UI elements (e.g., Pause/Resume).
    *   **Raycasting:** 3D object selection by clicking on the scene.

### 🎥 Camera System
*   **Orbit Camera:** Maya/Unity-style camera controls.
*   **Zoom & Pan:** Smooth zooming with mouse scroll and dynamic distance calculation.

## 🚀 Getting Started

### Prerequisites

1.  **Rust Toolchain:** Ensure you have Rust installed.
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **C/C++ Build Tools:** Required for compiling dependencies like `glfw-sys` and `russimp`.
    *   **Windows:** Visual Studio with C++ Desktop Development workload.
    *   **Linux:** `build-essential`, `cmake`, `libclang-dev`.
3.  **CMake:** Required for building `russimp`.

### Installation

1.  Clone the repository:
    ```sh
    git clone https://github.com/yourusername/opengl-renderer-rust.git
    cd opengl-renderer-rust
    ```

2.  Build the project (this may take a few minutes as it compiles dependencies):
    ```sh
    cargo build --release
    ```

3.  Run the application:
    ```sh
    cargo run --release
    ```

## 🎮 Controls

| Interaction | Action |
|-------------|--------|
| **Left Click + Drag** | Rotate Camera (Orbital) |
| **Mouse Scroll** | Zoom In / Out |
| **Shift + WASD** | Move Camera Focus Point (Pan) |
| **Left Click (on Object)** | Select Object (appears in Inspector) |
| **Inspector Panel** | Use `+` / `-` buttons to move selected object |
| **Pause Button** | Pause/Resume object animations |
| **ESC** | Close Application |

## 📂 Project Structure

*   `src/main.rs`: Entry point. Initializes the Window and Application loop.
*   `src/game/`: Contains the main game logic (`Game` struct), scene setup, and loop handling.
*   `src/renderer/`: Core rendering logic, shadow passes, and forward rendering pipeline.
*   `src/scene/`: Data structures for Scene, Objects, Materials, and Colliders.
*   `src/shaders/`: GLSL shader loading and program management.
*   `src/assets/`: Asset manager and path constants.
*   `src/importer/`: Model loading implementation (FBX/OBJ via Assimp).
*   `src/ui/`: UI system, Button, Inspector, and Text Rendering logic.
*   `src/input/`: Keyboard and Mouse input handling.
*   `assets/`: Directory containing shaders (`.vert`, `.frag`), models, and textures.

## 🛠️ Built With

*   [**gl**](https://crates.io/crates/gl): OpenGL function pointers.
*   [**glfw**](https://crates.io/crates/glfw): Window creation and input handling.
*   [**glam**](https://crates.io/crates/glam): Fast linear algebra (vectors, matrices).
*   [**russimp**](https://crates.io/crates/russimp): Rust bindings for the Assimp library (Asset Import).
*   [**image**](https://crates.io/crates/image): Loading texture images.
*   [**rusttype**](https://crates.io/crates/rusttype): Font loading and text rendering.

## 📜 License

This project is open-source and available under the **MIT License**.

---
*Created with ❤️ in Rust*
