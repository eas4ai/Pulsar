# Docs and Static Content

Pulsar's manual is Markdown-first and rendered through the app. This is the preferred user-facing documentation path. Rustdoc can document internals, but it should not be the manual people read to build a product.

## Manual Source

Manual files live in:

```text
content/docs/
```

The table of contents is:

```text
content/docs/documentation.md
```

Every Markdown link in that file becomes one docs chapter. The chapter slug comes from the file name, so `local-development.md` becomes `/docs/local-development`.

## Build Artifacts

Run:

```bash
cargo run --bin console -- docs:build
```

The command writes JSON artifacts to:

```text
storage/content/docs/
```

The app reads those artifacts at request time. Rebuild after changing chapter files or the table of contents.

## Links

Use relative Markdown links between chapters. For example, link to `authentication.md` in source Markdown; the builder rewrites that target to `/docs/authentication` in the rendered app.

## Search

The docs builder renders each chapter, extracts plain text, and writes a client-side search index into `catalog.json`. Good headings and direct wording make search better.

## Static Files

Files under `public/` are served by Suprnova's static fallback. Use `public/` for favicons, manifests, and files that should be available by path.

Continue with [Articles and RSS](articles-and-rss.md).
