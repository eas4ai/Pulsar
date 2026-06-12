# Configuration

Pulsar reads configuration from `.env` through Suprnova. Start from `.env.example` and keep secrets out of git.

## Required Local Keys

```bash
cp .env.example .env
openssl rand -base64 32 | tr '+/' '-_' | tr -d '='
```

Copy the generated value into `APP_KEY`.

## App Settings

- `APP_NAME` controls display naming in mail and app metadata.
- `APP_ENV` should be `local`, `test`, or `production`.
- `APP_DEBUG` should be `false` outside local development.
- `APP_URL` should match the public backend URL.

## Ports

Pulsar avoids common local ports by default:

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8765
VITE_PORT=5765
```

Set `SERVER_HOST=0.0.0.0` only when the app must accept external traffic.

## Database

Local development defaults to SQLite:

```env
DATABASE_URL=sqlite://./database.db
```

For production, use a managed database when possible:

```env
DATABASE_URL=postgres://user:password@host:5432/pulsar
```

## Mail

Use `MAIL_DRIVER=log` while developing. Switch to SMTP for real verification and reset mail.

```env
MAIL_DRIVER=smtp
MAIL_HOST=localhost
MAIL_PORT=1025
MAIL_FROM_ADDRESS=hello@example.com
MAIL_FROM_NAME="Pulsar"
```

Continue with [Authentication](authentication.md).
