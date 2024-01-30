# Commit Message Format Guidelines
## Overview

This document outlines the recommended commit message format for the project. 
Adopting a consistent format helps in better understanding the purpose of each commit 
and makes it easier to navigate through the project history.

## Commit Message Structure

A commit message should follow the structure:

```
[type] [summary] [Optional Flags] [Optional Details]
```

- **`[type]`:** Describes the type of the commit. Examples include `feat`, `fix`, `chore`, `docs`, `style`, `test`, `refactor`, `perf`, `build`, `ci`, `dep`, etc.

- **`[summary]`:** A brief, imperative statement summarizing the purpose of the commit.

- **`[Optional Flags]`:** Flags can be used to convey additional information. For example, use `[BREAKING CHANGE: ...]` to indicate breaking changes.

- **`[Optional Details]`:** Additional information or context about the changes made in the commit.

## Commit Types

### `feat` (Feature)

- **Usage:** Introducing a new feature for end-users.
- **Example:** `[feat] Add user authentication`

### `fix` (Bug Fix)

- **Usage:** Correcting issues or problems.
- **Example:** `[fix] Resolve null pointer issue`

### `chore` (Routine Tasks or Maintenance)

- **Usage:** Routine tasks, maintenance, or general refactoring.
- **Example:** `[chore] Refactor database connection handling`

### `docs` (Documentation)

- **Usage:** Changes related to documentation.
- **Example:** `[docs] Update installation instructions`

### `style` (Code Style)

- **Usage:** Code style changes, such as formatting or indentation.
- **Example:** `[style] Format code according to style guide`

### `test` (Testing)

- **Usage:** Adding or modifying tests.
- **Example:** `[test] Add unit tests for user authentication`

### `refactor` (Code Refactoring)

- **Usage:** Code restructuring or refactoring that doesn't change external behavior.
- **Example:** `[refactor] Extract common utility function`

### `perf` (Performance)

- **Usage:** Changes that improve the performance of the code.
- **Example:** `[perf] Optimize database query for faster response`

### `build` (Build System)

- **Usage:** Changes affecting the build system or external dependencies.
- **Example:** `[build] Update dependency versions`

### `ci` (Continuous Integration)

- **Usage:** Changes in the configuration or scripts of the continuous integration system.
- **Example:** `[ci] Update build pipeline`

### `dep` (Dependencies)

- **Usage:** Modifying or upgrading dependencies.
- **Example:** `[dep] Update third-party library to version X.Y.Z`

## Examples

### New Feature
```
feat: Add user authentication
```

### Bug Fix
```
fix: Resolve null pointer issue
```

### Breaking Change
```
feat: Introduce new feature [BREAKING CHANGE: Requires clients to update API calls with additional parameters] This commit adds a new feature for user authentication using OAuth2.
```

## Commit Stacking

Multiple commits can be stacked together to form a single commit message. Used when people forget to commit frequently

### Example

```
feat: Add user authentication
fix: Resolve null pointer issue
feat: Introduce new feature [BREAKING CHANGE: Requires clients to update API calls with additional parameters] This commit adds a new feature for user authentication using OAuth2.
style: Format code according to style guide
chore: Refactor database connection handling
```

## Notes

- Keep the summary under 150 characters, if possible.
- Provide as much detail as necessary in the `[Optional Details]` section.



By adhering to this commit message format, we maintain a clean and informative version history, making it easier for team members to understand the evolution of the project.