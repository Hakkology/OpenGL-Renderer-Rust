# Hakkology OpenGL Engine (Rust) 🦀

![Status](https://img.shields.io/badge/Status-Active-success)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)
![OpenGL](https://img.shields.io/badge/OpenGL-3.3%2B-blue)
![License](https://img.shields.io/badge/License-MIT-green)

![OpenGL Renderer](assets/resources/project/rust%20open%20gl%20screenshot.png)

A high-performance, modular 3D rendering engine built from scratch in Rust using modern OpenGL. This project serves as a showcase of advanced graphics programming techniques, including real-time dynamic lighting, shadow mapping, and a custom ECS-inspired scene architecture.

## 🌟 Key Features

### 🎨 Advanced Rendering Pipeline
*   **Dynamic Lighting System**:
    *   **Directional Lights**: Simulates sun/moon light with parallel rays.
    *   **Point Lights**: Omnidirectional lights with quadratic attenuation (e.g., light bulbs, fire).
    *   **Spot Lights**: Cone-shaped lights with soft edges (flashlight effect).
*   **High-Fidelity Shadows**:
    *   **Directional Shadows**: Implemented using high-res depth maps and **PCF (Percentage-Closer Filtering)** 3x3 sampling for soft shadow edges.
    *   **Omnidirectional Shadows**: Point lights cast shadows in all directions using **Dynamic Geometry Shader Cubemaps**.
*   **Material System**:
    *   **Blinn-Phong Shading**: Realistic specular highlights.
    *   **Texture Support**: Diffuse maps, UV tiling, and scaling.
    *   **Materials**: Support for `Gold`, `Emerald`, `Obsidian`, etc., via a preset factory.
*   **Skybox**: Seamless cubemap rendering for immersive backgrounds.

### 🏗 Engine Architecture
*   **Asset Management**: 
    *   Resource counting references (`Rc`) for efficient memory usage.
    *   Automatic caching of Shaders, Textures, and Models (OBJ) to prevent duplicate loading.
*   **Scene Graph**:
    *   **Transform Hierarchy**: Position, Rotation (Quaternions), and Scale.
    *   **Component System**: Objects can have attached `Colliders`, `Controllers` (scripts), and `Materials`.
*   **Input Handling**:
    *   Event-driven input system wrapping `GLFW` events.
    *   Raycasting for 3D object selection from screen space.

### 🎮 Interactive Elements
*   **Orbit Camera**: Professional CAD-like camera controls (Pan, Zoom, Orbit).
*   **Physics Lite**: Simple AABB and Sphere collision primitives.
*   **UI System**: Custom text rendering engine and batch-rendered 2D UI elements (buttons, panels).
*   **Logic Controllers**:
    *   `OrbitController`: For planetary motion.
    *   `FloatingController`: For "breathing" idle animations.
    *   `RotationController`: For constant spinning objects.

## 🛠 Project Structure

The codebase is organized into modular distinct crates/modules:

```bash
src/
├── assets/         # Asset Manager (Loaders for OBJ, PNG, GLSL)
├── config.rs       # Global Configuration (Window size, Light limits, Constants)
├── game/           # Core Game Loop & Scene Composition
├── input/          # Input State Management
├── light/          # Light Components (Directional, Point, Spot)
├── logic/          # Game Logic & Object Behaviours (Controllers)
├── math/           # Raycasting & Math Utilities
├── primitives/     # Procedural Mesh Generation (Cube, Sphere, Capsule, Plane)
├── renderer/       # Render Passes (Shadow Pass, Geometry Pass, Skybox Pass)
├── scene/          # Scene Graph, Objects, Materials
├── shaders/        # GLSL Shader Compilation & Linking
├── shapes/         # 2D Shapes
├── ui/             # User Interface (Text, Buttons)
└── window/         # Window Creation & Context Management
```

## 🚀 Getting Started

### Prerequisites

1.  **Rust Toolchain**: [Install Rust](https://www.rust-lang.org/tools/install)
2.  **C Compiler**: Required for compiling `glfw-sys`.
    *   *Windows*: Install Visual Studio C++ Build Tools.
    *   *Linux*: `sudo apt install build-essential cmake`
3.  **CMake**: Required for building GLFW.
    *   *Windows*: [Download CMake](https://cmake.org/download/) and add to PATH.

### Installation & Run

1.  Clone the repository:
    ```bash
    git clone https://github.com/Hakkology/OpenGL-Renderer-Rust.git
    cd OpenGL-Renderer-Rust
    ```

2.  Run in release mode for best performance:
    ```bash
    cargo run --release
    ```

> **Note**: First compilation might take a few minutes as it compiles dependencies like `glfw` and `image` crates.

## 🎮 Controls & Interactions

| Context | Input | Action |
|:-------:|:-----:|:-------|
| **Camera** | **LMB + Drag** | Orbit around the center |
| **Camera** | **Scroll** | Zoom In / Out |
| **Interaction** | **LMB Click** | Select Object (Raycast) |
| **Interaction** | **Pause Button** | Pause/Resume Object Animations |
| **System** | **Esc** | Close Application |

## 🧩 Modding & Configuration

You can tweak engine parameters in `src/config.rs` without touching core logic:

*   **`window`**: Resolution, Title, VSync.
*   **`camera`**: FOV, Sensitivity, Zoom Limits.
*   **`rendering`**: Shadow Map Resolution (Default: 2048), Max Lights.

## � License

This project is licensed under the [MIT License](LICENSE).

---

*Built with ❤️ by Hakkology & Antigravity AI*
