// next.config.js

/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    domains: ['localhost'], // Add localhost to the allowed domains
  },
};
const tauriNextConfig = {
  ...nextConfig,
  output: 'export',
};

module.exports = process.env.TAURI === 'true' ? tauriNextConfig : nextConfig;
