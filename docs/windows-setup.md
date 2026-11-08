# Windows Development Setup & OpenCV Build Prerequisites

This document outlines the environment configuration, build prerequisites, and verification steps for developing OMNISCAN on Windows (x86_64 MSVC).

---

## 1. Rust Toolchain Setup

OMNISCAN standardizes on the stable MSVC Rust toolchain.

### 1.1 Installation
Install Rust via `rustup`:
1. Download `rustup-init.exe` from [https://win.rustup.rs/x86_64](https://win.rustup.rs/x86_64).
2. Execute the installer:
   ```powershell
   & "$env:TEMP\rustup-init.exe" -y --default-host x86_64-pc-windows-msvc --default-toolchain stable
   ```
3. Verify that `%USERPROFILE%\.cargo\bin` is in your `PATH`:
   ```powershell
   rustc --version
   cargo --version
   ```

### 1.2 Toolchain Specification (`rust-toolchain.toml`)
The repository includes a `rust-toolchain.toml` pinned to:
- Channel: `stable`
- Components: `rustfmt`, `clippy`
- Target: `x86_64-pc-windows-msvc`

---

## 2. C++ & Build System Prerequisites

Building native extensions and future OpenCV bindings requires a C++ compiler, CMake, and LLVM/Clang.

| Tool | Recommended Version / Path | Purpose |
|---|---|---|
| **MSVC C++ Build Tools** | Visual Studio 2022/2026 (v143/v145) with C++ Desktop Workload | Host C/C++ compiler (`cl.exe`) and linker (`link.exe`) |
| **LLVM / Clang** | LLVM 18+ at `C:\Program Files\LLVM\bin` | Provides `libclang.dll` required by `clang-sys` and `bindgen` |
| **CMake** | CMake 3.25+ in `PATH` | Build configuration and native package discovery |
| **Git** | Git 2.40+ in `PATH` | Source control and Git hooks |

Set the `LIBCLANG_PATH` environment variable if not detected automatically:
```powershell
[System.Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\Program Files\LLVM\bin", [System.EnvironmentVariableTarget]::User)
```

---

## 3. OpenCV Native SDK Setup (Prerequisite for M2)

In M0, the core CLI and domain types are decoupled from native OpenCV, allowing immediate, lightweight builds. In **M2 (Image Ingestion)**, OpenCV bindings will be activated for image decoding and computer vision operations.

### 3.1 Option A: Pre-built Official Binaries (Recommended)
1. Download the official OpenCV Windows self-extracting archive (OpenCV 4.9.0 or 4.10.0) from GitHub releases:  
   `https://github.com/opencv/opencv/releases`
2. Extract to a stable directory, e.g., `C:\tools\opencv` or `C:\opencv`.
3. Configure environment variables (User or Machine):
   ```powershell
   # Point OPENCV_DIR to the build folder containing OpenCVConfig.cmake
   [System.Environment]::SetEnvironmentVariable("OPENCV_DIR", "C:\opencv\build", [System.EnvironmentVariableTarget]::User)

   # Add the runtime DLLs to PATH
   $currentPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User)
   $opencvBin = "C:\opencv\build\x64\vc16\bin"
   if ($currentPath -notlike "*$opencvBin*") {
       [System.Environment]::SetEnvironmentVariable("Path", "$opencvBin;$currentPath", [System.EnvironmentVariableTarget]::User)
   }
   ```
4. Verify discovery:
   ```powershell
   Test-Path "$env:OPENCV_DIR\OpenCVConfig.cmake"
   # Should return True
   ```

### 3.2 Option B: Microsoft vcpkg
Alternatively, install OpenCV via vcpkg:
```powershell
git clone https://github.com/microsoft/vcpkg.git C:\vcpkg
.\C\vcpkg\bootstrap-vcpkg.bat
.\C\vcpkg\vcpkg.exe install opencv4[core,imgproc,imgcodecs]:x64-windows
[System.Environment]::SetEnvironmentVariable("VCPKG_ROOT", "C:\vcpkg", [System.EnvironmentVariableTarget]::User)
```

---

## 4. Verification and Quality Commands

From the `omniscan` directory:

```powershell
# 1. Format verification
cargo fmt --check

# 2. Linter / Clippy verification (zero warnings policy)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Test suite execution
cargo test

# 4. CLI Execution
cargo run -- --help
cargo run -- validate-image sample.png
```

