import { getFileHash } from './getFileHash';

export async function getClassicAIVersion(): Promise<string> {
  // Only run in Node.js environment (server-side)
  if (typeof window === 'undefined' && typeof process !== 'undefined') {
    try {
      const { readFileSync } = await import('fs');
      const { createHash } = await import('crypto');

      // Get the Rust crate version
      const cargoTomlPath = 'worker/Cargo.toml';
      const cargoContent = readFileSync(cargoTomlPath, 'utf8');
      const versionMatch = cargoContent.match(/version = "([^"]+)"/);
      const crateVersion = versionMatch ? versionMatch[1] : '0.1.0';

      // Get a hash of the AI-specific source files to detect AI changes
      const aiSourceFiles = [
        'worker/src/lib.rs',
        'worker/src/features.rs',
        'worker/src/ml_ai.rs',
        'worker/src/neural_network.rs',
      ];

      const hash = createHash('sha256');
      for (const file of aiSourceFiles) {
        try {
          const content = readFileSync(file, 'utf8');
          hash.update(content);
        } catch {
          // File might not exist, continue
        }
      }

      const aiHash = hash.digest('hex').slice(0, 8);
      return `${crateVersion}-${aiHash}`;
    } catch {
      return 'unknown';
    }
  }
  return 'unknown';
}

export async function getMLAIVersion(): Promise<string> {
  return await getFileHash('public/ml-weights.json.gz');
}
