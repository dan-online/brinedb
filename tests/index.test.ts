import { afterAll, describe, expect, test } from "vitest";
import { Brine, BrineDatabases } from "..";

describe("Test suite", () => {
	let brine: Brine;

	test("can instantiate", async () => {
		brine = new Brine("mysql://root:root@localhost:3306/brine");
	});

	test("can initialize", async () => {
		expect(brine.get("test")).rejects.toThrowError("Brine not initialized");

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

	test("can clear", async () => {
		await brine.clear();

		const value = await brine.get("hello");

		expect(value).toBe(null);
	});

	test("can close", async () => {
		await brine.close();

		expect(brine.get("test")).rejects.toThrowError("Brine not initialized");
	});

	afterAll(async () => {
		await brine.close();
	});
});
