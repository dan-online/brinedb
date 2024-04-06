import bindings from "../native/brinedb.node";

export interface BrineOptions<T> {
	serialize?: (value: T) => string;
	deserialize?: (value: string) => T;
}

class Brine<T = unknown> {
	private readonly connectionURI: string;

	private initialized = false;

	private deserialize: (value: string) => T = JSON.parse;
	private serialize: (value: T) => string = JSON.stringify;

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

	public async init() {
		await bindings.migrate(this.connectionURI);

		this.initialized = true;
	}

	public async get(key: string): Promise<T | null> {
		const result = await this.internals
			.get(this.connectionURI, key)
			.catch(() => null);

		const parsed: T | null = result ? await this.deserialize(result) : null;

		return parsed;
	}

	public async set(key: string, value: T): Promise<T> {
		const serializedValue = this.serialize(value);

		await this.internals.set(this.connectionURI, key, serializedValue);

		return value;
	}

	public async clear() {
		await this.internals.clear(this.connectionURI);
	}

	public async delete(key: string): Promise<void> {
		await this.internals.del(this.connectionURI, key);
	}

	public async deleteMany(keys: string[]): Promise<void> {
		await this.internals.delMany(this.connectionURI, keys);
	}

	public async count() {
		const count = await this.internals.count(this.connectionURI);

		return count;
	}

	public async ensure(key: string, value: T): Promise<T> {
		const existing = await this.get(key);

		if (existing) {
			return existing;
		}

		return this.set(key, value);
	}

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
