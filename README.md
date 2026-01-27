## Project Goal 

> Fast, local-first note capture with safe sync

A resource-efficient, cross-platform desktop application (Tauri v2) that allows users to quickly capture notes via a global hotkey. 

It stores everything locally in SQLite for privacy and speed, supports optional manual sync to external providers (currently planned Notion), and automatically maintains old synced data so your local store stays tidy and efficient.

## Key features (To be implemented)

- Global hotkey capture for near-instant note entry
- Resource-efficient; minimal background footprint (Tauri + native UI)
- Local-first storage with SQLite (portable, fast, single-file)
- Manual sync to external providers (Notion support)
- Automatic maintenance of old synced entries (cleanup/archival rules)

## Dev Setup

Make sure Tauri dev prerequisites are satisfied. Refer: https://v2.tauri.app/start/prerequisites/

- Install packages
```
pnpm install
```

- Run `pnpm tauri dev` to start dev server


## License
MIT — see [LICENSE](LICENSE) for details.
