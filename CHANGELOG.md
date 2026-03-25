# Changelog

All notable changes to this project will be documented in this file.

## [0.7.0] - 2026-03-25

### ✨ Features

- **cli:** add bulk ops, task relations, comments, refactor resolve_task_id ([48afa22](https://github.com/ONREZA/kaneo-cli/commit/48afa2282d36b63a867487114104d880dd3e227a))
- **cli:** adapt to Kaneo v2.3.16, add archival, sorting, new resources ([ad3d72e](https://github.com/ONREZA/kaneo-cli/commit/ad3d72e332e4b4a12baf764fc7c917adf9937f8c))

## [0.6.0] - 2026-03-20

### ✨ Features

- **cli:** resolve human-readable task refs like DEP-65 ([af31105](https://github.com/ONREZA/kaneo-cli/commit/af31105ca08ac95056d13b88bdc316102017826e))

## [0.5.0] - 2026-03-14

### ✨ Features

- **cli:** adapt to Kaneo v2.3.11, add label attach/detach, server-side filtering, unit tests ([7937429](https://github.com/ONREZA/kaneo-cli/commit/7937429bebbc40617ac91c164a029455d7a6caa6))

## [0.4.1] - 2026-03-13

### 🐛 Bug Fixes

- **cli:** show requested status in task status output, clarify --status help ([70b6746](https://github.com/ONREZA/kaneo-cli/commit/70b6746c833b8627a6d1fdea57c4cbaa3ffd00eb))
- **cli:** use Value for delete responses instead of typed structs ([3ea81d3](https://github.com/ONREZA/kaneo-cli/commit/3ea81d3b4f30230cec6b8783fb7cf3ed382aa887))
- **cli:** remove positional project_id from create/import, add task list filters ([2922bfe](https://github.com/ONREZA/kaneo-cli/commit/2922bfeadb27f959ca53b28436a9fa6d6814e78e))
- **api:** correct workspace list endpoint to /auth/organization/list ([cd5dbf8](https://github.com/ONREZA/kaneo-cli/commit/cd5dbf83e300da7fd567213ba8b05fb9074af7f7))

### ✅ Testing

- **api:** add route validation against OpenAPI spec ([eb6b39c](https://github.com/ONREZA/kaneo-cli/commit/eb6b39c6344927f37903f5fc0cad9e0d4e993cf5))

## [0.4.0] - 2026-03-13

### ✨ Features

- **cli:** add interactive selection for link, status, priority, and set-active ([c727856](https://github.com/ONREZA/kaneo-cli/commit/c7278560b0059395a3a16dc7a49558d1823210ae))

## [0.3.0] - 2026-03-13

### ✨ Features

- **cli:** add .kaneo.json context system with link/unlink/context commands ([29a1778](https://github.com/ONREZA/kaneo-cli/commit/29a17780f1db542e857a2afa56676500bf2d9a34))

### 🐛 Bug Fixes

- **deps:** upgrade commitlint to v20, pin tinyexec 0.3.2 via override ([9221df1](https://github.com/ONREZA/kaneo-cli/commit/9221df194ea95709bbb4f04d6eb18beac9395ff5))

### ♻️ Changed

- **deps:** replace commitlint with cocogitto ([e22a38a](https://github.com/ONREZA/kaneo-cli/commit/e22a38aa74a80adf9c5c5809e1f1ce1500931eb3))

### 🔧 Changed

- **deps:** update all dependencies and GitHub Actions to latest ([d52acfb](https://github.com/ONREZA/kaneo-cli/commit/d52acfb14b4a76c9597a30aaeae9c0fd8dac8e11))

## [0.2.0] - 2026-03-13

### ✨ Features

- **cli:** add self-upgrade command and version update notifications ([dd6b2d6](https://github.com/ONREZA/kaneo-cli/commit/dd6b2d6d7ee59f2b8a8b0ba7b9566528fd6c1fb2))
- initial release — full Kaneo CLI ([71f16ae](https://github.com/ONREZA/kaneo-cli/commit/71f16ae2846567e9ea1a143d5b9e315ad6ceb12c))

### 🐛 Bug Fixes

- **ci:** fix formatting and clippy warnings, update CI to Node 24 ([54ae4c6](https://github.com/ONREZA/kaneo-cli/commit/54ae4c60a76023af38ef31a19fa0345f158366d9))

