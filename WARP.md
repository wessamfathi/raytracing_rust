# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

Project snapshot
- Crate: rtow_rust (Ray Tracing in One Weekend in Rust)
- Tooling: Cargo (Rust edition = 2024), dependencies: rand, rayon, overload
- Binary entrypoint: src/main.rs (outputs PPM to ./output/16.ppm)

Common workflows
```warp
name: Build (debug)
command: cargo build
tags:
  - rust
```

```warp
name: Build (release)
command: cargo build --release
tags:
  - rust
```

```warp
name: Run (debug)
command: cargo run
tags:
  - rust
  - run
```

```warp
name: Run (release)
description: Render using optimized build (recommended for speed)
command: cargo run --release
tags:
  - rust
  - run
```

```warp
name: Run (release, set Rayon threads)
description: Override Rayon thread count for reproducible performance
arguments:
  - name: threads
    defaultValue: "8"
command: env RAYON_NUM_THREADS=$threads cargo run --release
tags:
  - rust
  - rayon
  - performance
```

```warp
name: Format
command: cargo fmt --all
tags:
  - rust
  - lint
```

```warp
name: Lint (clippy)
command: cargo clippy --all-targets --all-features -D warnings
tags:
  - rust
  - lint
```

```warp
name: Lint (fmt + clippy)
description: Run rustfmt then clippy (fish shell safe sequencing)
command: cargo fmt --all; and cargo clippy --all-targets --all-features -D warnings
tags:
  - rust
  - lint
```

```warp
name: Clean build artifacts
command: cargo clean
tags:
  - rust
```

```warp
name: Test (all)
command: cargo test
tags:
  - rust
  - test
```

```warp
name: Test (single)
description: Run tests matching a filter (if/when tests exist)
arguments:
  - name: filter
    defaultValue: "your_test_filter"
command: cargo test $filter
tags:
  - rust
  - test
```

```warp
name: Open rendered image (macOS)
description: Open the last rendered PPM in the default viewer
command: open ./output/16.ppm
tags:
  - macos
  - output
  - viewer
```

High-level architecture
- src/main.rs
  - Orchestrates rendering: image sizing, camera setup, scene generation, and parallel render loop.
  - Uses Rayon to compute each image row in parallel, collects rows, sorts by scanline to preserve top-to-bottom output, and writes PPM to ./output/16.ppm.
  - Parameters baked-in: image_width=400, samples_per_pixel=100, MAX_DEPTH=50, ASPECT_RATIO from core::camera.
  - random_scene builds a ground sphere plus a grid of small spheres with randomized materials (Lambertian/Metal/Dielectric) and optional motion (time 0..1).

- src/lib.rs
  - Declares library modules: core, shapes, materials, re-exported for main.rs.

- core/* (fundamental math + camera + ray)
  - vec3.rs: Vec3 math with operator overloading (via overload) and helpers (unit_vector, cross, sqrt in-place, print to PPM color).
  - ray.rs: Ray with origin, direction, time; at(t) point evaluation.
  - hit_record.rs: Intersection payload (p, normal, t, front_face, material) with set_face_normal to orient normals.
  - camera.rs: Camera with DOF and motion blur parameters (aperture -> lens_radius, open_time/close_time). get_ray samples time in [open,close] enabling motion blur when objects move.
  - core.rs: Exposes submodules, math constants (PI, INFINITY), RNG helpers (random, random_in_unit_sphere), dot product.

- shapes/* (geometry and world)
  - sphere.rs: Static and moving sphere hit routines; supports material binding and motion via center0/center1 and ray.time.
  - hittable_list.rs: World container storing Spheres, linear hit traversal tracking closest hit.
  - hittable.rs: Placeholder base type; current implementation routes through concrete types (Sphere) rather than trait objects.

- materials/* (BSDFs)
  - lambertian.rs: Diffuse scatter using random_in_unit_sphere; returns albedo and scattered ray.
  - metal.rs: Reflective scatter with fuzziness and absorption when reflect dir leaves the surface.
  - dielectric.rs: Refractive/reflective glass using Schlick approximation and Snell’s law; always attenuates as white.

Repo-specific behavior and notes
- Output: Rendering writes a PPM file to ./output/16.ppm (FILE_PATH constant in main.rs). Ensure the output/ directory exists; the program does not create directories.
- Performance/parallelism: Rayon drives parallel row rendering. Control threads with env RAYON_NUM_THREADS (see workflow above). Sampling uses random() per thread; results are nondeterministic without a fixed seed.
- Camera & motion blur: Camera::new is called with open_time=0.0 and close_time=1.0. Some small spheres are marked moving; rays carry a time sampled within the shutter interval for motion blur.
- Edition: Cargo.toml specifies edition = 2024. Use an up-to-date toolchain compatible with this edition.
- Tests: No tests are present at this time; cargo test will report 0 tests.

Key files to start from
- src/main.rs: End-to-end rendering setup and entrypoint.
- src/core/camera.rs and src/core/vec3.rs: Ray generation and math utilities.
- src/shapes/sphere.rs and src/materials/*: Geometry and material interactions.
