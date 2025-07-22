
## 📋 Архитектура проекта

Проект состоит из нескольких модулей:

- `main.rs` — инициализация сервера, маршрутов и Swagger UI.
- `handlers/` — обработчики маршрутов (`actix_web`), включая аннотации `#[utoipa::path(...)]`.
- `models/` — DTO, структуры пользователя и ролей, используемые в API.
- `storage/` — логика чтения/записи пользователей и ролей из JSON.
- `frontend/` — статические HTML-страницы для взаимодействия с API.
- `api_docs.rs` — декларация всей OpenAPI-документации через `#[derive(OpenApi)]`.


## 🛡️ Важные замечания по безопасности

### ⚠️ Используется устаревшая библиотека

> ❗ `proc-macro-error` (через `utoipa-gen`) официально **не поддерживается**:
- Последний коммит — 2+ года назад
- Не отвечает на issue и email
- В `RustSec` помечен как unmaintained: [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370)

**Возможные альтернативы:**
- [`proc-macro2-diagnostics`](https://github.com/SergioBenitez/proc-macro2-diagnostics)
- [`manyhow`](https://crates.io/crates/manyhow)
- [`proc-macro-error2`](https://crates.io/crates/proc-macro-error2)

> ❗ На текущий момент библиотека всё ещё используется в `utoipa`, поэтому её нельзя просто удалить без полной замены OpenAPI-генератора.

---

## ✅ Пример успешного CI-вывода

```txt
✅ cargo check
✅ cargo test
⚠️ 1 warning (proc-macro-error: unmaintained)
✅ cargo deny check bans (с предупреждениями о дубликатах)
