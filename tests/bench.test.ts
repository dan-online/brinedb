import { describe, test } from "vitest";
import { Bench } from "tinybench";
import { Brine, BrineDatabases } from "../src";

const randomData = (length: number) => {
	const chars =
		"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let result = "";

	for (let i = 0; i < length; i++) {
		result += chars.charAt(Math.floor(Math.random() * chars.length));
	}

	return result;
};

describe("Benchmark", () => {
	test("Brine", async () => {
		const bench = new Bench({ time: 200 });
		const brinedb = new Brine(BrineDatabases.sqlite.file("test.sqlite"), {
			serialize: (value) => value as string,
			deserialize: (value) => value,
		});

		await brinedb.init();
		await brinedb.clear();

		const setInitialManyData: [string, string][] = [];
		const size = 1_000;

		await new Promise<void>((done) => {
			for (let i = 0; i < size; i++) {
				setInitialManyData.push([`key-${i}`, randomData(100)]);
			}

			done();
		});

		await brinedb.setMany(setInitialManyData);

		bench
			.add("get", async () => {
				await brinedb.get(`key-${Math.floor(Math.random() * size)}`);
			})
			.add("set", async () => {
				await brinedb.set(
					`key-${Math.floor(Math.random() * size)}`,
					randomData(100),
				);
			})
			.add("getMany", async () => {
				await brinedb.getMany(setInitialManyData.map(([key]) => key));
			})
			.add("keys", async () => {
				await brinedb.keys();
			})
			.add("values", async () => {
				await brinedb.values();
			})
			.add("count", async () => {
				await brinedb.count();
			});

		await bench.warmup();
		await bench.run();

		console.table(bench.table());

		await brinedb.close();
	});
});
