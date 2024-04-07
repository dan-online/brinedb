declare module "*.node" {
	export function get(
		connectionURI: string,
		key: string,
	): Promise<string | null>;
	export function set(
		connectionURI: string,
		key: string,
		value: string,
	): Promise<undefined>;
	export function clear(connectionURI: string): Promise<undefined>;
	export function migrate(connectionURI: string): Promise<undefined>;
	export function del(connectionURI: string, key: string): Promise<undefined>;
	export function delMany(
		connectionURI: string,
		keys: string[],
	): Promise<undefined>;
	export function count(connectionURI: string): Promise<number>;
	export function has(connectionURI: string, key: string): Promise<boolean>;
	export function close(): Promise<undefined>;
}
