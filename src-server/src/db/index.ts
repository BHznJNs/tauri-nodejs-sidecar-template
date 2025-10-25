import { drizzle } from "drizzle-orm/libsql";

const DEFAULT_DB_FILE_NAME = "file:local.db";

export const db = drizzle(DEFAULT_DB_FILE_NAME);
