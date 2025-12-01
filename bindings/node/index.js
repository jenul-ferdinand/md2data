// Platform-aware loader for multi-platform .node bindings
const { platform, arch } = process;

// Map Node.js platform/arch to Rust target triple
const PLATFORMS = {
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'win32-x64': 'x86_64-pc-windows-msvc',
};

const platformKey = `${platform}-${arch}`;
const target = PLATFORMS[platformKey];

if (!target) {
  throw new Error(
    `Unsupported platform: ${platformKey}. ` +
    `Supported platforms: ${Object.keys(PLATFORMS).join(', ')}`
  );
}

const nativeBinding = `index.${target}.node`;

try {
  module.exports = require(`./${nativeBinding}`);
} catch (err) {
  throw new Error(
    `Failed to load native binding for ${platformKey}. ` +
    `Expected file: ${nativeBinding}. ` +
    `Original error: ${err.message}`
  );
}
