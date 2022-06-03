# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `btreelist` macro [#8](https://github.com/jeffa5/btreelist/pull/8)
- `swap` function to swap two indices [#10](https://github.com/jeffa5/btreelist/pull/10)

### Changed

- Made the `B` parameter exposed as a `const generic` on the list [#5](https://github.com/jeffa5/btreelist/pull/5)

## [0.3.0] - 2022-06-02

### Added

- Implementations of `Index` and `IndexMut` for `BTreeList`

### Changed

- Remove panics, instead returning `Option`s or `Result`s

## [0.2.0] - 2022-06-02

### Added

- More documentation
- `first` and `last` methods on `BTreeList`
- `pop` method on `BTreeList`
- Owned iterator

## [0.1.1] - 2022-06-01

### Removed

- `Clone` and `Debug` bounds on implementation

## [0.1.0] - 2022-06-01

### Added

- Initial data structure

[unreleased]: https://github.com/jeffa5/btreelist/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/jeffa5/btreelist/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jeffa5/btreelist/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/jeffa5/btreelist/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jeffa5/btreelist/releases/tag/v0.1.0
