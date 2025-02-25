// next.config.js

/** @type {import('next').NextConfig} */
const nextConfig = {};
const tauriNextConfig = {
  ...nextConfig,
  output: 'export',
};

module.exports = process.env.TAURI === 'true' ? tauriNextConfig : nextConfig;
