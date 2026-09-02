# Pull Request

> [!IMPORTANT]
> **CDDM is developed on a self-hosted Gitea forge (Primary SSoT).**
> This GitHub repository serves as a downstream read-only replica mirror.
> Please open and manage your Pull Request on our primary portal:
> **👉 https://git.gt-web-dev.com/gt-dev/cddm/pulls**

## Description

Summarize the changes made and the motivation behind them.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation update
- [ ] Tree-sitter grammar support

## Checklist

- [ ] My code follows the code style of this project (`cargo fmt`).
- [ ] I have run `cargo clippy` without warnings.
- [ ] I have added tests that prove my fix is effective or that my feature works.
- [ ] All new and existing unit tests pass locally (`cargo test` / `bun run test`).
- [ ] Documentation has been updated accordingly.
- [ ] CDDM Dogfooding self-scan passes (`cddm scan . --min-tokens 50`).
