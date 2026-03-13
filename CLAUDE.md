# kaneo-cli

CLI для [Kaneo](https://kaneo.app) — минималистичного таск-трекера. Поддерживает как cloud (cloud.kaneo.app), так и self-hosted инстансы.

## Сборка и запуск

```bash
cargo build              # dev build
cargo build --release    # release build (LTO + strip)
cargo run -- --help      # запустить с аргументами
cargo fmt                # форматирование
cargo clippy -- -D warnings  # линтинг
```

## Архитектура

```
src/
├── main.rs              # entry point, command routing
├── output.rs            # dual output: JSON (pipe/CI) + human (TTY + colors)
├── api/
│   ├── client.rs        # reqwest HTTP client (auth + anonymous + S3 upload)
│   ├── types.rs         # Rust structs для всех API-ответов Kaneo (ручные, из valibot-схем)
│   └── mod.rs
├── auth/
│   └── mod.rs           # profiles, token resolution (flags → env → config file)
├── cli/
│   ├── mod.rs           # clap derive: все команды и аргументы
│   ├── *_handler.rs     # обработчик для каждого ресурса
│   └── api_check_handler.rs  # валидация CLI vs server OpenAPI spec
└── config/
    └── mod.rs           # зарезервировано для project-level config
```

### Паттерны

- **Dual output**: `--json` (авто при pipe/не-TTY) vs `--human` (цвета через `console` crate). Вывод в stderr для human, stdout для JSON data.
- **Auth resolution**: `--token` → `KANEO_API_KEY` env → `~/.config/kaneo/config.json` профили. Каждый профиль — отдельный инстанс (URL + key + workspace).
- **API client**: reqwest + rustls (без OpenSSL). Отдельная функция `upload_to_presigned_url()` для S3 без auth headers.
- **api-check**: таблица `EXPECTED_OPERATIONS` в `api_check_handler.rs` — при добавлении нового эндпоинта, добавлять маппинг туда.

## Kaneo API

- Backend: Hono (TypeScript) на порту 1337
- Auth: better-auth с API Key plugin (Bearer token в Authorization header)
- Workspaces: через better-auth Organization plugin (`/api/auth/organization/*`)
- Файлы: presigned S3 URLs (upload) + `/api/asset/:id` (download)
- OpenAPI spec: `/api/openapi` (генерируется динамически, доступен без auth)

## Как добавлять новый эндпоинт

1. Добавить типы в `src/api/types.rs` (если нужны новые структуры)
2. Добавить subcommand в `src/cli/mod.rs`
3. Добавить handler в `src/cli/<resource>_handler.rs`
4. Зароутить в `src/main.rs`
5. Добавить operationId маппинг в `src/cli/api_check_handler.rs` → `EXPECTED_OPERATIONS`

## Conventions

- Файлы/модули/функции: `snake_case`
- Типы: `PascalCase`
- Serde: `#[serde(rename_all = "camelCase")]` для совместимости с Kaneo API
- Ошибки: `anyhow` для приложения, `thiserror` если нужен typed error
- Алиасы команд: `ls` для list, `rm` для delete, короткие имена ресурсов (`t`, `proj`, `col`, `notif`, `ws`, `time`)
