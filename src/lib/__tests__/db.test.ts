import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

vi.mock('@opennextjs/cloudflare', () => ({
  getCloudflareContext: vi.fn(),
}));

vi.mock('drizzle-orm/d1', () => ({
  drizzle: vi.fn(() => 'd1-db'),
}));

vi.mock('drizzle-orm/better-sqlite3', () => ({
  drizzle: vi.fn(() => 'sqlite-db'),
}));

vi.mock('better-sqlite3', () => ({
  default: vi.fn(function () {
    return 'sqlite-instance';
  }),
}));

describe('Database', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.resetModules();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  describe('getDb - development environment', () => {
    beforeEach(() => {
      vi.stubEnv('NODE_ENV', 'development');
    });

    it('should create SQLite database in development', async () => {
      const { getDb } = await import('../db');
      const db = await getDb();
      expect(db).toBe('sqlite-db');
    });

    it('should reuse existing database instance', async () => {
      const { getDb } = await import('../db');
      const db1 = await getDb();
      const db2 = await getDb();
      expect(db1).toBe(db2);
    });

    it('should handle SQLite creation error', async () => {
      const Database = (await import('better-sqlite3')).default;
      vi.mocked(Database).mockImplementation(() => {
        throw new Error('SQLite error');
      });

      const { getDb } = await import('../db');
      await expect(getDb()).rejects.toThrow('SQLite error');
    });
  });

  describe('getDb - production environment', () => {
    beforeEach(() => {
      vi.stubEnv('NODE_ENV', 'production');
    });

    it('should create D1 database in production', async () => {
      const { getCloudflareContext } = await import('@opennextjs/cloudflare');
      const { drizzle: drizzleD1 } = await import('drizzle-orm/d1');

      const mockEnv = { DB: 'd1-binding' };
      vi.mocked(getCloudflareContext).mockResolvedValue({ env: mockEnv } as any);

      const { getDb } = await import('../db');
      const db = await getDb();

      expect(getCloudflareContext).toHaveBeenCalledWith({ async: true });
      expect(drizzleD1).toHaveBeenCalledWith('d1-binding', expect.any(Object));
      expect(db).toBe('d1-db');
    });

    it('should handle missing DB binding', async () => {
      const { getCloudflareContext } = await import('@opennextjs/cloudflare');

      const mockEnv = {};
      vi.mocked(getCloudflareContext).mockResolvedValue({ env: mockEnv } as any);

      const { getDb } = await import('../db');
      await expect(getDb()).rejects.toThrow('DB binding is not available in Cloudflare context');
    });

    it('should handle Cloudflare context error', async () => {
      const { getCloudflareContext } = await import('@opennextjs/cloudflare');
      vi.mocked(getCloudflareContext).mockRejectedValue(new Error('Cloudflare error'));

      const { getDb } = await import('../db');
      await expect(getDb()).rejects.toThrow('Cloudflare error');
    });

    it('should reuse existing database instance', async () => {
      const { getCloudflareContext } = await import('@opennextjs/cloudflare');
      const { drizzle: drizzleD1 } = await import('drizzle-orm/d1');

      const mockEnv = { DB: 'd1-binding' };
      vi.mocked(getCloudflareContext).mockResolvedValue({ env: mockEnv } as any);

      const { getDb } = await import('../db');
      const db1 = await getDb();
      const db2 = await getDb();

      expect(db1).toBe(db2);
      expect(getCloudflareContext).toHaveBeenCalledTimes(1);
      expect(drizzleD1).toHaveBeenCalledTimes(1);
    });
  });

  describe('error handling', () => {
    it('should propagate errors from D1 setup', async () => {
      vi.stubEnv('NODE_ENV', 'production');
      const { getCloudflareContext } = await import('@opennextjs/cloudflare');
      const { drizzle: drizzleD1 } = await import('drizzle-orm/d1');

      const mockEnv = { DB: 'd1-binding' };
      vi.mocked(getCloudflareContext).mockResolvedValue({ env: mockEnv } as any);
      vi.mocked(drizzleD1).mockImplementation(() => {
        throw new Error('D1 setup error');
      });

      const { getDb } = await import('../db');
      await expect(getDb()).rejects.toThrow('D1 setup error');
    });

    it('should propagate errors from SQLite setup', async () => {
      vi.stubEnv('NODE_ENV', 'development');
      const { drizzle: drizzleSqlite } = await import('drizzle-orm/better-sqlite3');
      vi.mocked(drizzleSqlite).mockImplementation(() => {
        throw new Error('SQLite error');
      });

      const { getDb } = await import('../db');
      await expect(getDb()).rejects.toThrow('SQLite error');
    });
  });
});
