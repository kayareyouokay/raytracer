# Rust Ray Tracer

A simple ray tracer implemented in Rust, based on the "Ray Tracing in One Weekend" book by Peter Shirley.

## Features

- Ray-sphere intersection
- Lambertian (diffuse) and metal materials
- Antialiasing
- Depth of field
- Multithreaded rendering

## Requirements

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Building and Running

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust_raytracer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the ray tracer:
   ```bash
   cargo run --release
   ```

   The output will be saved as `output.png` in the project directory.

## Controls

- The scene is predefined in `main.rs`
- You can modify the scene by editing the `main` function in `src/main.rs`
- Adjust image quality by changing `SAMPLES_PER_PIXEL` and `MAX_DEPTH` constants

## Output

The ray tracer will generate an image file named `output.png` in the project directory. The image will show a simple scene with several spheres on a large ground plane.

## Learning Resources

- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
