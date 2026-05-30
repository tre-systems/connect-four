import { drizzle as drizzleD1, DrizzleD1Database } from 'drizzle-orm/d1';
import { drizzle as drizzleSqlite, BetterSQLite3Database } from 'drizzle-orm/better-sqlite3';
import Database from 'better-sqlite3';
import * as schema from './schema';

let db: DrizzleD1Database<typeof schema> | BetterSQLite3Database<typeof schema> | null = null;

export const getDb = async () => {
  if (!db) {
    if (process.env.NODE_ENV === 'production') {
      const { getCloudflareContext } = await import('@opennextjs/cloudflare');
      const { env } = await getCloudflareContext({ async: true });

      if (!env.DB) {
        throw new Error('DB binding is not available in Cloudflare context');
      }

      db = drizzleD1(env.DB, { schema });
    } else {
      const sqlite = new Database('local.db');
      db = drizzleSqlite(sqlite, { schema });
    }
  }

  return db;
};
