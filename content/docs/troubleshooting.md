# Troubleshooting

This chapter covers the failures you are most likely to hit while running Pulsar locally.

## The App Starts on the Wrong Port

Check `.env`:

```env
APP_URL=http://localhost:8765
SERVER_PORT=8765
VITE_PORT=5765
```

If your installed CLI is stale, reinstall Suprnova from the framework repo:

```bash
cd /home/shawn/workspace/nation-x-com
cargo install --path suprnova-cli --force --root ~/.local
```

## Docs Route Says Artifacts Are Missing

Run:

```bash
cargo run --bin console -- docs:build
```

The docs route reads from `storage/content/docs`, not directly from `content/docs`.

## Frontend Type Errors After Serve

Regenerate types and run the checker:

```bash
suprnova generate-types
cd frontend && bun run check
```

Generated page props live in `frontend/src/types/inertia-props.ts`. Shared nested DTO declarations live in `frontend/src/types/inertia-shared.d.ts`.

## Login Works But Dashboard Redirects

The dashboard requires a verified email address. In local development, verification mail is logged when `MAIL_DRIVER=log`. Copy the verification URL from the backend log and open it in the browser.

## Admin Pages Return Forbidden

Admin pages require both a verified account and permissions. Promote a user:

```bash
cargo run --bin console -- users:promote --email user@example.com --role admin
```

Then log out and back in so the session reflects the updated role.

## Blog Is Empty

Seed articles:

```bash
cargo run --bin console -- articles:seed
```

If articles exist but do not appear, confirm they are published.

You now have the full Pulsar manual path. Return to [Getting Started](getting-started.md) when onboarding a new project.
