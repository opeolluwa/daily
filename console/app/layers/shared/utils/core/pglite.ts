import { PGlite } from "@electric-sql/pglite";

const migrationModules = import.meta.glob<{
  default: string;
}>("../../data/core/migrations/*.sql", {
  eager: true,
  query: "?raw",
  import: "default",
});

let dbPromise: Promise<PGlite> | null = null;

/**
 * Returns the shared PGlite instance for the "core" in-browser database,
 * applying any pending core migrations on first use. Persists to IndexedDB.
 */
export function coreDb(): Promise<PGlite> {
  if (!dbPromise) {
    dbPromise = createCoreDb();
  }
  return dbPromise;
}

async function createCoreDb(): Promise<PGlite> {
  const db = new PGlite("idb://core");

  await db.exec(
    `CREATE TABLE IF NOT EXISTS seaql_migrations ("version" TEXT NOT NULL PRIMARY KEY, "applied_at" BIGINT NOT NULL);`,
  );

  const applied = new Set<string>(
    (
      await db.query<{ version: string }>(
        "SELECT version FROM seaql_migrations",
      )
    ).rows.map((row) => row.version),
  );

  const versions = Object.keys(migrationModules).sort();
  if (versions.length === 0) {
    throw new Error(
      "[core] no migration SQL bundled — run `node scripts/sync-core-schema.mjs` before building",
    );
  }
  for (const key of versions) {
    const version = key
      .split("/")
      .pop()!
      .replace(/\.sql$/, "");
    if (applied.has(version)) continue;

    const sql = migrationModules[key]!.trim();
    if (!sql || sql === `-- ${version}`) continue;

    await db.exec("BEGIN;");
    try {
      await db.exec(sql);
      await db.query<never>(
        "INSERT INTO seaql_migrations (version, applied_at) VALUES ($1, $2);",
        [version, Date.now()],
      );
      await db.exec("COMMIT;");
    } catch (error) {
      await db.exec("ROLLBACK;");
      throw error;
    }
  }

  return db;
}

/** Closes and discards the cached database instance (for tests / resets). */
export async function resetCoreDb(): Promise<void> {
  if (dbPromise) {
    const db = await dbPromise;
    await db.close();
    dbPromise = null;
  }
}
