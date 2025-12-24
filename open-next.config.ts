import { defineCloudflareConfig } from '@opennextjs/cloudflare';

const sharedOverride = {
  converter: 'edge',
  proxyExternalRequest: 'fetch',
  tagCache: 'dummy',
  queue: 'dummy',
} as const;

export default defineCloudflareConfig({
  default: {
    override: { ...sharedOverride, wrapper: 'cloudflare-node' },
  },
  middleware: {
    external: true,
    override: { ...sharedOverride, wrapper: 'cloudflare-edge' },
  },
  assets: { patterns: ['public/wasm/*'] },
  edgeExternals: ['node:crypto'],
} as any);
