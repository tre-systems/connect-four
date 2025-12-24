import { defineCloudflareConfig } from '@opennextjs/cloudflare';

/**
 * @type {import("@opennextjs/cloudflare").OpenNextConfig}
 */
const config = {
  default: {
    override: {
      wrapper: 'cloudflare-node',
      converter: 'edge',
      proxyExternalRequest: 'fetch',
      tagCache: 'dummy',
      queue: 'dummy',
    },
  },
  assets: {
    patterns: ['public/wasm/*'],
  },
  edgeExternals: ['node:crypto'],
  middleware: {
    external: true,
    override: {
      wrapper: 'cloudflare-edge',
      converter: 'edge',
      proxyExternalRequest: 'fetch',

      tagCache: 'dummy',
      queue: 'dummy',
    },
  },
};

export default defineCloudflareConfig(config as any);
