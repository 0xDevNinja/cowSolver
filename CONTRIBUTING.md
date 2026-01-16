# Contributing to cowSolver

Thank you for your interest in improving **cowSolver**! Contributions are welcome and appreciated. These guidelines help maintain a high-quality and well-organized codebase.

## Reporting Issues

- Use GitHub issues to report bugs, suggest enhancements or request new features.
- Include as much detail as possible (steps to reproduce, expected vs. actual behaviour, environment info).
- Search existing issues before opening a new one to avoid duplicates.

## Pull Requests

- Fork this repository and create a feature branch (e.g., `feature/your-feature-name`).
- Write clear, descriptive commit messages and reference relevant issues using keywords like `Closes #<issue_number>`.
- Follow the existing code style and run `cargo fmt` before committing to ensure consistent formatting.
- Ensure the project builds and all tests pass (`cargo test`). Add new tests to cover your changes.
- Update documentation and README files when adding new modules or altering existing behaviour.
- Submit your pull request against the `master` branch with a detailed description of your changes and any additional context.

## Coding Standards

- The project uses **Rust 2021** and adheres to idiomatic Rust practices.
- Use explicit error types with `Result<T, E>` for error handling. Avoid using `unwrap()` or panics in library code.
- Keep functions small and focused; break complex logic into smaller units when possible.
- Run `clippy` to catch potential issues and follow its suggestions where appropriate.

## Communication & Conduct

- Be respectful and constructive in discussions. We strive to maintain an inclusive and collaborative community.
- Participate in code reviews by providing thoughtful feedback and suggestions for improvement.
- If you have questions about the code or architecture, feel free to open a discussion or reach out via issues.

## License

By contributing to this project, you agree that your contributions will be licensed under the [MIT License](LICENSE).
