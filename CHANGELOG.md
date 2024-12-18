# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.5](https://github.com/beaumontjonathan/companies-house-api-rs/compare/v0.0.4...v0.0.5) - 2024-11-27

### Added

- *(streaming-api)* support basic streaming api for companies and filings
- *(public-data-api)* support better errors from operation requests

### Fixed

- *(public-data-api)* various small company profile and search data changes to support rare cases
- *(public-data-api)* make all data struct fields public

### Other

- *(docs)* tweak import formatting of example script

## [0.0.4](https://github.com/beaumontjonathan/companies-house-api-rs/compare/v0.0.3...v0.0.4) - 2024-11-13

### Added

- *(public-data-api)* add new operation `SearchDisqualifiedOfficers`

### Fixed

- *(public-data-api)* check status codes on all responses before attempting to deserialize body

### Other

- *(public-data-api)* impove examples
- *(public-data-api)* fix incorrect doc comments for `OfficerSearch`

## [0.0.3](https://github.com/beaumontjonathan/companies-house-api-rs/compare/v0.0.2...v0.0.3) - 2024-11-11

### Added

- *(public-data-api)* add new operation `SearchOfficers`

### Fixed

- *(public-data-api)* move `matches`, `snippet` and `title` into `CompanySearchItem` struct

## [0.0.2](https://github.com/beaumontjonathan/companies-house-api-rs/compare/v0.0.1...v0.0.2) - 2024-11-11

### Added

- *(public-data-api)* add new operation `SearchCompanies`

### Fixed

- *(public-data-api)* mark `made_up_to: Date` as optional for `GetCompanyProfile`

### Other

- add commitlint / commitizen / husky support
- set vscode cspell config ot en-GB and rust-analyzer check command to clippy
- add release-plz GitHub Actions config
- *(project)* tweak `README.md` formatting
