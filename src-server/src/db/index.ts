import { drizzle } from "drizzle-orm/better-sqlite3";

const DEFAULT_DB_FILE_NAME = "sqlite.db";

export const db = drizzle(DEFAULT_DB_FILE_NAME);
