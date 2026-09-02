# Nature of Code in Rust 🦀

> Porting Daniel Shiffman's *"The Nature of Code"* examples, simulations, and exercises to Rust using the **Macroquad** game engine. Huge thanks to Daniel for his incredible teaching!

---

## 📂 Project Structure

This project is structured as a single **Cargo Workspace**, sharing a high-performance vector math crate and an interactive simulation runner:

* **`runner/`**: Shared interactive application loop, 2D camera control system, HUD panels, and rendering helpers.
* **`vec_math/`**: Custom `Vector2` library, Perlin Noise wrapper, and zero-GC Quadtree implementation for spatial partitioning.
* **`chapters/`**: Chapter implementation crates:
  * `chapter_0_randomness`: Random walks, probability distributions, Monte Carlo selection, and Perlin noise.
  * `chapter_1_vectors`: Vector math, magnitude, normalization, and Motion 101 dynamics.
  * `chapter_2_forces`: Newton's laws ($F=ma$), friction, drag forces, N-body attraction, and Barnes-Hut Quadtree optimization.
  * `chapter_3_oscillation`: Angles, angular motion, harmonic oscillation, wave dynamics, and Hooke's Law spring physics.

---

## 📊 Progress & Topic Checklist

### Chapter 0: Randomness (`-p chapter_0_randomness`)
- [x] **0.1** Standard 4-way & 8-way Random Walkers
- [x] **0.2** Biased & Rightward Tendency Random Walkers
- [x] **0.3** Uniform Random Distribution Visualization
- [x] **0.4** Gaussian / Normal Distribution Simulation
- [x] **0.5** Custom Accept-Reject (Monte Carlo) Distribution
- [x] **0.6** 1D Perlin Noise Graphing
- [x] **0.7** 2D Perlin Noise Terrain / Smooth Surface Generation
- [x] **0.8** Smooth Perlin Noise Walker

### Chapter 1: Vectors (`-p chapter_1_vectors`)
- [x] **1.1** Bouncing Ball (Scalar vs. Vector implementation)
- [x] **1.2** Vector Operations (Addition, Subtraction, Multiplication, Division)
- [x] **1.3** Vector Magnitude & Normalization ($\hat{v}$)
- [x] **1.4** Random Unit Vectors & Mouse Direction Vectors
- [x] **1.5** Motion 101: Velocity Integration ($P_{t+1} = P_t + V$)
- [x] **1.6** Motion 101: Acceleration Integration ($V_{t+1} = V_t + A$)
- [x] **1.7** Motion 101: Random & Mouse-Attracted Acceleration

### Chapter 2: Forces (`-p chapter_2_forces`)
- [x] **2.1** Force Accumulation & Newton's Second Law ($\Sigma F = m a$)
- [x] **2.2** Mass Scaling & Gravity Simulation
- [x] **2.3** Friction Forces ($f = -\mu N \hat{v}$)
- [x] **2.4** Fluid Drag & Resistance ($F_d = -\frac{1}{2}\rho v^2 A C_d \hat{v}$)
- [x] **2.5** Gravitational Attraction (Single Attractor & Two-Body System)
- [x] **2.6** Mutual N-Body Gravitational Attraction
- [x] **2.7** Barnes-Hut $O(N \log N)$ Spatial Quadtree Acceleration

### Chapter 3: Oscillation (`-p chapter_3_oscillation`)
- [x] **3.1 — Angles & Rotation**
  - [x] Radians & trigonometric vector rotation
  - [x] Baton Rotation Simulation (`3.1`)
- [x] **3.2 — Angular Motion**
  - [x] Angular displacement ($\theta$), velocity ($\omega$), and acceleration ($\alpha$)
  - [x] Euler-style angular integration ($\theta_{t+1} = \theta_t + \omega$, $\omega_{t+1} = \omega_t + \alpha$)
  - [x] Interactive Mouse Drag, Spin & Damping Baton Simulation (`3.2.1`)
  - [x] Force-driven Angular Motion & N-Body Attractor Simulation (`3.2.2`)
  - [x] CannonBall with Spin - impulse force, continuous downward gravity, and initial spin (`3.2.3`)
- [x] **3.3 — Pointing in the Direction of Motion**
  - [x] Calculating heading orientation angle ($\theta = \text{atan2}(v_y, v_x)$)
  - [x] Vehicle acceleration toward target mouse position (`3.3`)
  - [x] Interactive Vehicle Steering Simulation with WASD / Arrow Keys (`Exercise 3.4`)
- [ ] **3.x — Harmonic Motion & Oscillations**
  - [ ] Sine & Cosine trig functions
  - [ ] Amplitude, Period, and Frequency ($x = A \cdot \sin(2\pi f t)$)
  - [ ] Simple Harmonic Motion (SHM)
  - [ ] `Oscillator` struct & independent X/Y oscillations (Lissajous curves)
  - [ ] Varying Amplitudes & Angular Velocities
- [ ] **Waves**
  - [ ] Static Sine Wave plot
  - [ ] Animated Wave simulation
  - [ ] Wavelength ($\lambda$) & Phase shift ($\phi$)
  - [ ] Additive Waves (wave superposition)
- [ ] **Spring Forces & Dynamics**
  - [ ] Hooke's Law ($F_s = -k x$)
  - [ ] Spring + Bob physics system
  - [ ] Gravity + Spring equilibrium physics
  - [ ] Inclined-Plane exercise (Vector breakdown + Friction + Gravity)

---

## 🚀 Getting Started

Make sure you have [Rust](https://www.rust-lang.org/) installed. 

To run a specific chapter's simulation runner, use Cargo's `-p` flag:

```bash
cargo run -p chapter_0_randomness
cargo run -p chapter_1_vectors
cargo run -p chapter_2_forces
cargo run -p chapter_3_oscillation
```

### 🎮 Interactive Controls

When the simulation window opens, use the built-in runner controls:

* **Left / Right Arrow Keys:** Switch between different examples in the chapter.
* **`r` Key:** Reset the current example state.

---

## 🛠️ Tech Stack

* **Language:** [Rust (2021 Edition)](https://www.rust-lang.org/)
* **Graphics & Windowing:** [Macroquad](https://macroquad.rs/)
* **Source Material:** [The Nature of Code](https://natureofcode.com/) by Daniel Shiffman
