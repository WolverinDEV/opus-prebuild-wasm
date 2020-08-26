# Opus prebuild wasm  
This repository contains a prebuild `libopus.a` usable for the `wasm32-unknown-unknown` target.  
  
## Opus library build steps  
Sadly I've not really documented my steps (it was late and more a poc thingy).  
Nevertheless, here is a ruff protocol how I did it.  
  
### 1. LLVM Setup
Setup LLVM supporting the `wasm32-unknown-unknown-wasm` target (Should be supported by default nowadays)  

### 2. Emscripten
Download Emscripten (We require the libc build for wasm)  
Alternatively you could use something like this: https://github.com/WebContainer/musl-wasm  
(We only need the libc from emscripten)  

### 3. Compile Opus
Compile the Opus CMake project with these parameters:  
| Variable | Description |  
| -- | -- |  
| `$EM_CACHE` | Points to the emscripten cache directory containing the `libc.a` (only after you've build something? idk...) |  
| `$EM_UPSTREAM` | Points to the current emscripten upstream folder   |  

Example: 
```
export EM_CACHE = "/mnt/c/Users/WolverinDEV/.emscripten_cache/wasm/"
export EM_UPSTREAM = "/mnt/d/git/web/emsdk/upstream/"
```  
  
CMake compiler definitions:  
| CMake variable | Parameter | Description |   
| -- | -- | -- |   
| `CMAKE_C_FLAGS` | `--target=wasm32-unknown-unknown-wasm` | Directing clang to use `wasm32-unknown-unknown-wasm` as the build target |  
| `CMAKE_C_FLAGS` | `-emit-llvm` | We need to emit the llvm ir code for later processing |  
| `CMAKE_C_FLAGS` | `-L $EM_CACHE -l c` | Link the emscripten c library (if you're using musl, change this) |  
| `CMAKE_C_FLAGS` | `-I$EM_UPSTREAM/emscripten/system/include/libc -I$EM_UPSTREAM/emscripten/system/lib/libc/musl/arch/emscripten` | Including the clib headers |  

Full command line:
```
cmake . -DCMAKE_C_FLAGS="--target=wasm32-unknown-unknown-wasm -emit-llvm -L $EM_CACHE -l c -I$EM_UPSTREAM/emscripten/system/include/libc -I$EM_UPSTREAM/emscripten/system/lib/libc/musl/arch/emscripten" -DCMAKE_C_COMPILER_WORKS=1
```