# Frontend and Design System

Pulsar uses Vue 3, Inertia, Vuetify, MDI icons, and a custom design layer in `frontend/src/app.css`.

## Page Flow

Rust controllers return Inertia page names such as `Home`, `Dashboard`, `docs/Show`, or `admin/articles/Edit`. The frontend resolves those names to Vue files under `frontend/src/pages`.

Example:

- Controller page name: `articles/Show`
- Vue file: `frontend/src/pages/articles/Show.vue`

## Design Tokens

The live design system is centered in:

- `frontend/src/app.css`
- `frontend/src/plugins/vuetify.ts`
- shared components in `frontend/src/components`

Prefer the existing classes and Vuetify variants before adding new styling. Keep product surfaces quiet, readable, and dense enough for repeated use.

## Generated Props

Suprnova generates page prop types into:

```text
frontend/src/types/inertia-props.ts
```

Do not hand-edit that file. If a page needs shared nested DTOs, add declarations to:

```text
frontend/src/types/inertia-shared.d.ts
```

## Frontend Commands

```bash
cd frontend
bun run dev -- --host 127.0.0.1 --port 5765
bun run check
bun run build
```

## Adding a Page

1. Add a route in `src/routes.rs`.
2. Add a controller handler and `#[derive(InertiaProps)]` props.
3. Add a matching Vue page under `frontend/src/pages`.
4. Run `suprnova generate-types` or `suprnova serve`.
5. Run `cd frontend && bun run check`.

Continue with [Docs and Static Content](docs-and-content.md).
