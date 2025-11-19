# Contributing to Tauri Telephony Plugin

Thank you for your interest in contributing to the Tauri Telephony Plugin! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/tauri-telephony-plugin.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes thoroughly
6. Commit your changes: `git commit -m "Add your descriptive commit message"`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Node.js 20 or higher
- Android Studio (for Android development)
- Xcode (for iOS development, macOS only)

### Building the Project

```bash
# Install dependencies
npm install

# Build Rust crate
cargo build

# Build TypeScript bindings
npm run build

# Run tests
cargo test
npm test
```

### Testing on Mobile

#### Android

1. Open the `android` directory in Android Studio
2. Sync Gradle files
3. Run tests: `./gradlew test`
4. Run instrumented tests on a device/emulator: `./gradlew connectedAndroidTest`

#### iOS

1. Open the `ios` directory in Xcode
2. Select a simulator or device
3. Run tests: `swift test` or use Xcode's test runner (Cmd+U)

## Code Style

### Rust

- Follow the official Rust style guidelines
- Run `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Add documentation comments for public APIs

### TypeScript

- Use ESLint configuration provided in the repository
- Run `npm run lint` before committing
- Use TypeScript strict mode
- Add JSDoc comments for exported functions

### Kotlin (Android)

- Follow Kotlin coding conventions
- Use 4 spaces for indentation
- Add KDoc comments for public APIs

### Swift (iOS)

- Follow Swift API Design Guidelines
- Use 4 spaces for indentation
- Add documentation comments for public APIs

## Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or modifications
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

Example: `feat: add support for call recording on Android`

## Pull Request Guidelines

1. **Title**: Use a clear, descriptive title
2. **Description**: Explain what changes you made and why
3. **Tests**: Include tests for new features or bug fixes
4. **Documentation**: Update documentation if needed
5. **Breaking Changes**: Clearly mark any breaking changes

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Tests pass locally
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
- [ ] Commits follow conventional commit format

## Testing Guidelines

### Unit Tests

- Write unit tests for all new functions
- Aim for high code coverage
- Test edge cases and error conditions

### Integration Tests

- Test platform-specific implementations on actual devices
- Verify permission handling works correctly
- Test event emission and handling

### End-to-End Tests

- Test the example application on real devices
- Verify all features work as expected
- Check for memory leaks and performance issues

## Reporting Issues

When reporting issues, please include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Detailed steps to reproduce the problem
3. **Expected Behavior**: What you expected to happen
4. **Actual Behavior**: What actually happened
5. **Environment**:
   - OS and version
   - Device/emulator information
   - Plugin version
   - Tauri version
6. **Logs**: Relevant error messages or logs

## Feature Requests

We welcome feature requests! Please:

1. Check if the feature has already been requested
2. Clearly describe the feature and its use case
3. Explain why it would be beneficial
4. Provide examples if possible

## Code Review Process

1. All submissions require review
2. Maintainers will review PRs as soon as possible
3. Address feedback and update your PR
4. Once approved, a maintainer will merge your PR

## Community

- Be respectful and considerate
- Help others when you can
- Follow the [Code of Conduct](./CODE_OF_CONDUCT.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.

## Questions?

Feel free to open an issue or discussion if you have questions about contributing!

Thank you for contributing to Tauri Telephony Plugin! 🎉
