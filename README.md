# Sphere Sandbox

![](https://github.com/user-attachments/assets/bbfb5550-edee-48e2-917e-a849d5e189bf)

Welcome to the **Simple Sphere Sandbox** project! This is a Rust-based application that allows users to interact with simple physics simulations involving spheres. You can summon dynamic and static spheres and restart the simulation at any time.

Sphere Properties
- **Color:** The color of each sphere indicates its level of damping. A more intense color reflects higher damping, while a most blue shade indicates bigger damping.
- **Size:** The size of the sphere (calculated using the area formula) represents its weight. Larger spheres are heavier and will behave differently under simulated physics.


## Controls
- **Right Mouse Button (RMB)**: Summon a dynamic sphere at the cursor position.
- **Control + Right Mouse Button (Ctrl + RMB)**: Summon a static sphere at the cursor position.
- **R Key**: Restart the simulation.

## Installation
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/simple-sphere-sandbox.git
   ```

3. Navigate to the project directory:
   ```bash
   cd simple-sphere-sandbox
   ```

4. Build the project:
   ```bash
   cargo build --release
   ```

5. Run the application:
   ```bash
   cargo run
   ```

## Contributing
If you'd like to contribute to the project, please fork the repository and submit a pull request. I welcome improvements, bug fixes, and new features!

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

Happy sandboxing! ^w^   
