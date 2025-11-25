# Scripts

Utility scripts for bootstrapping Monkey language tooling live here. Each language-specific
subdirectory mirrors the same responsibilities so you can choose whichever runtime you prefer.

## Structure

- `bash/setup-rust.sh` &mdash; POSIX shell helper that installs Rust through Homebrew, configures the
  stable toolchain, and adds common components like `rustfmt` and `clippy`.
- `ts/setup-rust.ts` &mdash; TypeScript version of the same workflow. Run it via `pnpm ts-node` (or
  after compiling) if you prefer a Node-based environment.

## Usage Examples

```bash
# Bash variant
./scripts/bash/setup-rust.sh

# TypeScript variant
pnpm ts-node scripts/ts/setup-rust.ts
```

> Note: Both scripts expect Homebrew to be installed and will exit early with guidance if it is missing. The
> TypeScript version surfaces more granular error messages thanks to typed helpers but otherwise
> performs the exact same steps as the Bash script.
