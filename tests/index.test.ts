import { afterAll, describe, expect, test } from "vitest";
import { Brine, BrineDatabases } from "../src";

describe("Brine", () => {
	let brine: Brine;

	test("can instantiate", async () => {
		brine = new Brine(BrineDatabases.sqlite.memory);
	});

	test("can instantiate w/ custom serialization", async () => {
		const serialize = (value: string) => value;
		const deserialize = (value: string) => value;

		new Brine(BrineDatabases.sqlite.memory, {
			serialize,
			deserialize,
		});
	});

	test("can initialize", async () => {
		expect(brine.get("test")).rejects.toThrowError("No connection found");

		await brine.init();
	});

	test("can clear empty", async () => {
		await brine.clear();
	});

	test("can set a value", async () => {
		const value = await brine.set("hello", "world");

		expect(value).toBe("world");
	});

	test("can set a complex value", async () => {
		const value = await brine.set("hello_complex", {
			hello: "world",
		});

		expect(value).toEqual({
			hello: "world",
		});
	});

	test("can set overwrite a value", async () => {
		const value = await brine.set("hello", "world");

		expect(value).toBe("world");
	});

	test("can get a value", async () => {
		const value = await brine.get("hello");

		expect(value).toEqual("world");

		const value_complex = await brine.get("hello_complex");

		expect(value_complex).toEqual({
			hello: "world",
		});
	});

	test("can get a null value", async () => {
		const value = await brine.get("key");

		expect(value).toBe(null);
	});

	test("can delete a value", async () => {
		await brine.set("delete", "this");
		await brine.delete("delete");

		const value = await brine.get("delete");

		expect(value).toBe(null);
	});

	test("can delete many values", async () => {
		await brine.set("delete1", "this");
		await brine.set("delete2", "this");
		await brine.set("delete3", "this");

		await brine.deleteMany(["delete1", "delete2", "delete3"]);

		const value1 = await brine.get("delete1");
		const value2 = await brine.get("delete2");
		const value3 = await brine.get("delete3");

		expect(value1).toBe(null);
		expect(value2).toBe(null);
		expect(value3).toBe(null);
	});

	test("can count", async () => {
		const count = await brine.count();

		expect(count).toBe(2);
	});

	test("can ensure", async () => {
		const value = await brine.ensure("ensure", "this");

		expect(value).toBe("this");

		const value2 = await brine.ensure("ensure", "that");

		expect(value2).toBe("this");
	});

	test("can has", async () => {
		const value = await brine.has("hello");

		expect(value).toBe(true);

		const value2 = await brine.has("key");

		expect(value2).toBe(false);
	});

	test("can get many", async () => {
		const data = await brine.getMany(["hello", "key"]);

		expect(data).toEqual({
			hello: "world",
			key: null,
		});
	});

	test("can clear", async () => {
		await brine.clear();

		const value = await brine.get("hello");

		expect(value).toBe(null);
	});

	test("can set many", async () => {
		const data: [string, string][] = [
			["key1", "value1"],
			["key2", "value2"],
			["key3", "value3"],
		];

		await brine.setMany(data);

		const value1 = await brine.get("key1");
		const value2 = await brine.get("key2");
		const value3 = await brine.get("key3");

		expect(value1).toBe("value1");
		expect(value2).toBe("value2");
		expect(value3).toBe("value3");
	});

	test("can keys", async () => {
		const keys = await brine.keys();

		expect(keys).toEqual(["key1", "key2", "key3"]);
	});

	test("can values", async () => {
		const values = await brine.values();

		expect(values).toEqual(["value1", "value2", "value3"]);
	});

	test("can close", async () => {
		await brine.close();

		expect(brine.get("test")).rejects.toThrowError("No connection found");
	});

	afterAll(async () => {
		await brine.close();
	});
});

describe("BrineDatabase", () => {
	describe("sqlite", () => {
		test("memory", async () => {
			expect(BrineDatabases.sqlite.memory).toEqual("sqlite::memory:");
		});

		test("file", async () => {
			expect(BrineDatabases.sqlite.file("test.db")).toEqual(
				"sqlite:test.db?mode=rwc",
			);
		});
	});

	describe("mysql", () => {
		test("build", async () => {
			expect(
				BrineDatabases.mysql.build({
					user: "user",
					password: "pass",
					database: "brinedb",
				}),
			).toEqual("mysql://user:pass@localhost:3306/brinedb");
		});

		test("build with host/port", async () => {
			expect(
				BrineDatabases.mysql.build({
					user: "user",
					password: "pass",
					database: "brinedb",
					host: "localhost",
					port: 3307,
				}),
			).toEqual("mysql://user:pass@localhost:3307/brinedb");
		});
	});

	describe("postgres", () => {
		test("build", async () => {
			expect(
				BrineDatabases.postgres.build({
					user: "user",
					password: "pass",
					database: "brinedb",
				}),
			).toEqual("postgres://user:pass@localhost:5432/brinedb");
		});

		test("build with host/port", async () => {
			expect(
				BrineDatabases.postgres.build({
					user: "user",
					password: "pass",
					database: "brinedb",
					host: "localhost",
					port: 5433,
				}),
			).toEqual("postgres://user:pass@localhost:5433/brinedb");
		});
	});
});
