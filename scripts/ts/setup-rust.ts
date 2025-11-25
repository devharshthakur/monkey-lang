#!/usr/bin/env node

/**
 * Homebrew-only Rust bootstrapper implemented in TypeScript so it can be run with pnpm.
 * Mirrors the previous Bash script: ensure brew exists, install rustup-init,
 * configure the stable toolchain, add rustfmt/clippy, and print tool versions.
 */

import { spawnSync, type SpawnSyncOptions } from 'node:child_process';

const log = (msg: string): void => {
  console.log(`[setup-rust] ${msg}`);
};

// Helper wrapper that surfaces non-zero exits immediately.
const run = (cmd: string, args: string[], opts: SpawnSyncOptions = {}): void => {
  const result = spawnSync(cmd, args, { stdio: 'inherit', ...opts });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${cmd} ${args.join(' ')}`);
  }
};

// Lightweight existence check executed with suppressed stdio.
const commandExists = (cmd: string, args: string[] = ['--version']): boolean => {
  const result = spawnSync(cmd, args, { stdio: 'ignore' });
  return result.status === 0;
};

// Fail fast when brew isn't present—everything else depends on it.
const ensureHomebrew = (): void => {
  if (!commandExists('brew', ['--version'])) {
    log('Homebrew is required but not found. Install it from https://brew.sh/');
    process.exit(1);
  }
};

// Install rustup-init via Homebrew only once.
const installRustup = (): void => {
  const alreadyInstalled =
    spawnSync('brew', ['list', 'rustup-init'], { stdio: 'ignore' }).status === 0;
  if (alreadyInstalled) {
    log('rustup-init already installed');
    return;
  }

  log('Installing rustup-init via Homebrew');
  run('brew', ['install', 'rustup-init']);
};

// Configure the stable toolchain and required components, running rustup-init if needed.
const initializeRust = (): void => {
  const rustupReady =
    commandExists('rustup', ['--version']) &&
    spawnSync('rustup', ['show', 'active-toolchain'], { stdio: 'ignore' }).status === 0;

  if (!rustupReady) {
    log('Bootstrapping Rust toolchain with rustup-init');
    run('rustup-init', ['-y', '--no-modify-path']);
  } else {
    log('Rust toolchain already configured');
  }

  log('Ensuring stable toolchain and common components');
  run('rustup', ['default', 'stable']);
  run('rustup', ['component', 'add', 'rustfmt', 'clippy']);
};

// Emit final versions for visibility and debugging.
const verifySetup = (): void => {
  log('Installed tool versions:');
  run('rustc', ['--version']);
  run('cargo', ['--version']);
  run('rustup', ['--version']);
};

// Orchestrates the full Homebrew + rustup flow with basic error reporting.
const main = (): void => {
  try {
    ensureHomebrew();
    log('Updating Homebrew formulae');
    run('brew', ['update']);
    installRustup();
    initializeRust();
    verifySetup();
    log('Rust setup via Homebrew completed');
  } catch (err) {
    if (err instanceof Error) {
      console.error(err.message);
    } else {
      console.error('Unknown error', err);
    }
    process.exit(1);
  }
};

main();
