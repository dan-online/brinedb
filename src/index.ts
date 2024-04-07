import bindings from "../native/brinedb.node";

export interface BrineOptions<T> {
	/**
	 * Custom serialization function
	 *
	 * @param value The value to serialize
	 * @default JSON.stringify
	 * @returns The serialized value
	 */
	serialize?: (value: T) => string | Promise<string>;
	/**
	 * Custom deserialization function
	 * @param value The value to deserialize
	 * @default JSON.parse
	 * @returns The deserialized value
	 */
	deserialize?: (value: string) => T | Promise<T>;
}

/**
 * Brine is a simple key-value store that persists data using Rust SeaORM bindings
 *
 * @example
 * ```ts
 * import { Brine } from "brinedb";
 *
 * const brine = new Brine("sqlite::memory:");
 *
 * await brine.init();
 *
 * await brine.set("key", "value");
 * const value = await brine.get("key");
 * ```
 */
class Brine<T = unknown> {
	private readonly connectionURI: string;

	private initialized = false;

	private deserialize: (value: string) => T | Promise<T> = JSON.parse;
	private serialize: (value: T) => string | Promise<string> = JSON.stringify;

	/**
	 * Create a new Brine instance
	 *
	 * @param connectionURI The connection URI to the database
	 * @param options Options for custom serialization and deserialization
	 */
	constructor(connectionURI: string, options?: BrineOptions<T>) {
		this.connectionURI = connectionURI;

		if (options?.serialize) {
			this.serialize = options.serialize;
		}

		if (options?.deserialize) {
			this.deserialize = options.deserialize;
		}
	}

	private get internals() {
		this.initCheck();

		return bindings;
	}

	/**
	 * Initialize the Brine instance and run migrations
	 *
	 * @returns Promise<void>
	 *
	 * @example
	 * ```ts
	 * await brinedb.init();
	 * ```
	 */
	public async init() {
		await bindings.migrate(this.connectionURI);

		this.initialized = true;
	}

	/**
	 * Get a value from the store
	 *
	 * @param key The key to get
	 * @returns The value or null if it doesn't exist
	 *
	 * @example
	 * ```ts
	 * const value = await brinedb.get("key");
	 * ```
	 */
	public async get(key: string): Promise<T | null> {
		const result = await this.internals
			.get(this.connectionURI, key)
			.catch(() => null);

		const parsed: T | null = result ? await this.deserialize(result) : null;

		return parsed;
	}

	/**
	 * Set a value in the store
	 *
	 * @param key The key to set
	 * @param value The value to set
	 * @returns The value that was set
	 *
	 * @example
	 * ```ts
	 * await brinedb.set("key", { foo: "bar" });
	 * await brinedb.set("key", "value");
	 * ```
	 */
	public async set(key: string, value: T): Promise<T> {
		const serializedValue = await this.serialize(value);

		await this.internals.set(this.connectionURI, key, serializedValue);

		return value;
	}

	/**
	 * Clear all values from the database
	 *
	 * @returns Promise<void>
	 *
	 * @example
	 * ```ts
	 * await brinedb.clear();
	 * ```
	 */
	public async clear() {
		await this.internals.clear(this.connectionURI);
	}

	/**
	 * Delete a key from the database
	 *
	 * @param key The key to delete
	 * @returns Promise<void>
	 *
	 * @example
	 * ```ts
	 * await brinedb.delete("key");
	 * ```
	 */
	public async delete(key: string): Promise<void> {
		await this.internals.del(this.connectionURI, key);
	}

	/**
	 * Delete multiple keys from the database
	 *
	 * @param keys The keys to delete
	 * @returns Promise<void>
	 *
	 * @example
	 * ```ts
	 * await brinedb.deleteMany(["key1", "key2"]);
	 * ```
	 */
	public async deleteMany(keys: string[]): Promise<void> {
		await this.internals.delMany(this.connectionURI, keys);
	}

	/**
	 * Count all documents in the database
	 *
	 * @returns Promise<number>
	 *
	 * @example
	 * ```ts
	 * const count = await brinedb.count();
	 * ```
	 */
	public async count() {
		const count = await this.internals.count(this.connectionURI);

		return count;
	}

	/**
	 * Ensure a key exists in the database
	 *
	 * @param key The key to check
	 * @param value The value to set if the key doesn't exist
	 * @returns The value that was set or the existing value
	 *
	 * @example
	 * ```ts
	 * const value = await brinedb.ensure("key", "value");
	 *
	 * value === "value"; // true
	 *
	 * const changed = await brinedb.ensure("key", "changed");
	 *
	 * changed === "value"; // true
	 * ```
	 */
	public async ensure(key: string, value: T): Promise<T> {
		const existing = await this.get(key);

		if (existing) {
			return existing;
		}

		return this.set(key, value);
	}

	/**
	 * Check if a key exists in the database
	 *
	 * @param key The key to check
	 * @returns Promise<boolean>
	 * @example
	 * ```ts
	 * const exists = await brinedb.has("key");
	 * ```
	 */
	public async has(key: string): Promise<boolean> {
		const value = await this.internals.has(this.connectionURI, key);

		return value;
	}

	/**
	 * Set many keys in the database
	 *
	 * @param data An array of 2d arrays containing keys and values
	 * @returns Promise<void>
	 */
	public async setMany(data: [string, T][]) {
		const serializedData = await Promise.all(
			data.map(async ([key, value]) => [key, await this.serialize(value)]),
		);

		await this.internals.setMany(
			this.connectionURI,
			serializedData as [string, string][],
		);
	}

	/**
	 * Close the connection to the database
	 *
	 * @returns Promise<void>
	 *
	 * @example
	 * ```ts
	 * await brinedb.close();
	 * ```
	 */
	public async close() {
		if (!this.initialized) return;

		await this.internals.close();

		this.initialized = false;
	}

	private initCheck() {
		if (!this.initialized) {
			throw new Error("Brine not initialized");
		}
	}
}

export { Brine };
export default Brine;
export * from "./utils";
