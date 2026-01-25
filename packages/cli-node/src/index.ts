#!/usr/bin/env node
/**
 * opencode-cloud Node.js CLI
 *
 * This is a transparent wrapper that spawns the Rust binary.
 * All arguments are passed through directly.
 */

import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Resolve binary path relative to this script
// When running from dist/index.js, binary should be at ../bin/occ
const scriptDir = dirname(fileURLToPath(import.meta.url));
const binaryPath = join(scriptDir, '..', 'bin', 'occ');

// Spawn the Rust binary with all arguments passed through
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit', // Pass through stdin/stdout/stderr for colors, TTY detection
});

// Handle process exit
child.on('close', (code) => {
  process.exit(code ?? 1);
});

// Handle binary not found or other spawn errors
child.on('error', (err) => {
  console.error('Error: Failed to spawn opencode-cloud binary\n');
  console.error(`Binary path: ${binaryPath}`);
  console.error(`Error: ${err.message}\n`);
  console.error('The Rust binary is not available. You have two options:\n');
  console.error('1. Install the Rust CLI directly:');
  console.error('   cargo install opencode-cloud\n');
  console.error('2. For development, copy the binary to packages/cli-node/bin/:');
  console.error('   cp target/release/occ packages/cli-node/bin/\n');
  console.error(`Platform: ${process.platform} (${process.arch})`);
  process.exit(1);
});
