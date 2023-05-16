import pg from "pg";
import { drizzle } from 'drizzle-orm/node-postgres';

const Pool = pg.Pool;

const pgPool = new Pool({
  connectionString: process.env.DATABASE_URL,
});

export const db = drizzle(pgPool);

export function closeDbPool() {
  return pgPool.end();
}
