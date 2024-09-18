# TODOS

- Fix the Dockerfile
  - Use a multi-stage build to avoid having the Rust image be my final base
  - Avoid cargo binstall, and compile trunk from source for security
- Setup CI for tests/lints
