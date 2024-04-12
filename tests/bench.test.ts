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
	test(
		"Brine",
		async () => {
			const bench = new Bench({ time: 200 });
			const brinedb = new Brine(BrineDatabases.sqlite.file("test.sqlite"));

			brinedb.init();
			brinedb.clear();

			const setInitialManyData: [string, string][] = [];
			const size = 1_000;

			for (let i = 0; i < size; i++) {
				setInitialManyData.push([`key-${i}`, randomData(10)]);
			}

			await brinedb.setMany(setInitialManyData);

			bench
				.add("get", async () => {
					await brinedb.get(`key-${Math.floor(Math.random() * size)}`);
				})
				.add("set", async () => {
					await brinedb.set(
						`key-${Math.floor(Math.random() * size) + size}`,
						randomData(100),
					);
				})
				.add("getMany", async () => {
					await brinedb.getMany(
						setInitialManyData.slice(0, 100).map(([key]) => key),
					);
				})
				.add("keys", () => {
					brinedb.keys();
				})
				.add("values", () => {
					brinedb.values();
				})
				.add("count", () => {
					brinedb.count();
				});

			await bench.warmup();
			await bench.run();

			const table = bench.table();
			const newTable: {
				[key: string]: string | number;
			}[] = [];

			for (const row of table) {
				if (!row) continue;

				newTable.push({
					...row,
					"Average Time (ms)": (row["Average Time (ns)"] as number) / 1_000_000,
					"Average Time (ns)": (row["Average Time (ns)"] as number).toFixed(2),
				});
			}

			console.table(newTable);

			brinedb.close();
		},
		{
			timeout: 10000,
		},
	);
});
