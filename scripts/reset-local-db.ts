import { existsSync, unlinkSync } from 'fs';
import { drizzle } from 'drizzle-orm/better-sqlite3';
import Database from 'better-sqlite3';
import { migrate } from 'drizzle-orm/better-sqlite3/migrator';

const dbPath = './local.db';

console.log('Resetting local database...');

if (existsSync(dbPath)) {
  console.log('Removing existing database...');
  unlinkSync(dbPath);
}

console.log('Creating new database...');
const sqlite = new Database(dbPath);
const db = drizzle(sqlite);

console.log('Applying migrations...');
migrate(db, { migrationsFolder: './migrations' });

console.log('Database reset complete!');
sqlite.close();
