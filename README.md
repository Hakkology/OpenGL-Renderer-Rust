# OpenGL Renderer in Rust

A modular and performant 3D rendering engine built with **Rust** and **OpenGL**. This project demonstrates advanced rendering techniques, including real-time lighting, shadow mapping, model loading, and a flexible camera system.

## 🚀 Key Features

### 🎨 Rendering & Graphics
- **Advanced Shaders**: Custom GLSL shaders for various material types:
  - **Phong Lighting**: Implements Ambient, Diffuse, and Specular reflection components.
  - **Textured Models**: Support for diffuse and specular maps.
  - **Skybox**: High-quality cubemap rendering for immersive backgrounds.
  - **Shadow Mapping System**:
  - **Directional Shadows**: Parallel light shadows for sun-like lighting.
  - **Omnidirectional Shadows**: 360-degree real-time shadows for point lights (using cubemaps and geometry shaders).
  - **Optimized Rendering**: Interleaved shadow updates and selective depth pass rendering for high performance.
- **Model Import**: Robust 3D model loading using **Assimp** (via `russimp`), supporting formats like `.obj`, `.blend`, `.fbx`, etc.
- **Texture Support**: Loads standard image formats (JPG, PNG, TIFF) as textures.

### 🎥 Camera & Controls
- **Orbit Camera**: Smooth 3rd-person camera control to inspect scenes.
- **Zoom**: Mouse wheel support for dynamic field-of-view adjustment.
- **Input Handling**: Integrated mouse and keyboard event system.
- **Raycasting**: Interaction system to detect objects under the cursor (e.g., "Raycast Hit: 'Wall +Z'").

### 🛠 System Architecture
- **Modular Design**: Codebase organized into distinct modules (`game`, `light`, `shaders`, `camera`, etc.) for maintainability.
- **UI System**: Custom `TextRenderer` for drawing debug information and UI overlays.
- **Component-Based Lights**: Flexible lighting system supporting multiple light sources.

---

## 💻 Build & Setup

To build and run this project, you need the **Rust** toolchain installed.

### System Dependencies

This project relies on `russimp` (Assimp bindings) and `glfw`, which require system-level C++ libraries to be built from source.

#### Ubuntu / Debian (Linux)

You must install build tools and libraries manually before running `cargo run`:

```bash
sudo apt-get update
# Essential build tools and libraries for Assimp, GLFW, and Windowing
sudo apt-get install cmake clang libclang-dev ninja-build build-essential libxi-dev libxcursor-dev libxinerama-dev libxrandr-dev xorg-dev
```

#### Windows

1.  **Visual Studio C++ Build Tools** (VS 2019 or 2022) must be installed.
2.  **CMake** must be installed and added to your system `PATH`.
3.  The project is configured in `Cargo.toml` to **statically link** Assimp by building it from source via a Git dependency patch.

## ▶️ How to Run

```bash
cargo run
```

*Note: The first build will take some time as it compiles `Assimp` from source.*

## 🎮 Controls

- **Mouse Move**: Rotate Camera (Orbit)
- **Scroll**: Zoom In/Out
- **Shift + Move**: Sprint (2x Speed)
- **Esc**: Exit Application
