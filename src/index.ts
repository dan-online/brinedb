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
 * @param connectionURI The connection URI to the database
 * @param options Options for custom serialization and deserialization
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
	 */
	public async clear() {
		await this.internals.clear(this.connectionURI);
	}

	/**
	 * Delete a key from the database
	 *
	 * @param key The key to delete
	 * @returns Promise<void>
	 */
	public async delete(key: string): Promise<void> {
		await this.internals.del(this.connectionURI, key);
	}

	/**
	 * Delete multiple keys from the database
	 *
	 * @param keys The keys to delete
	 * @returns Promise<void>
	 */
	public async deleteMany(keys: string[]): Promise<void> {
		await this.internals.delMany(this.connectionURI, keys);
	}

	/**
	 * Get all keys in the database
	 *
	 * @returns Promise<number>
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
	 */
	public async has(key: string): Promise<boolean> {
		const value = await this.internals.has(this.connectionURI, key);

		return value;
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
