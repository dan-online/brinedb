import { Brine } from "../../dist";
import { Spinner } from "@favware/colorette-spinner";
import { BrineDatabases } from "../../src";
import { Bench } from "tinybench";

const randomData = (length: number) => {
	const chars =
		"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let result = "";

	for (let i = 0; i < length; i++) {
		result += chars.charAt(Math.floor(Math.random() * chars.length));
	}

	return result;
};

type BrineLike = {
	init: () => Promise<void>;
	clear: () => Promise<void>;
	set: (key: string, value: unknown) => Promise<unknown>;
	get: (key: string) => Promise<unknown>;
	setMany: (data: [string, string][]) => Promise<void>;
	count: () => Promise<number>;
	close: () => Promise<void>;
};

const benchme = async (name: string, db: BrineLike) => {
	const spinner = new Spinner("Initializing database");
	const bench = new Bench({ time: 1000, warmupTime: 500 });

	spinner.start();

	await db.init();

	spinner.update({
		text: "Setting up database",
	});

	await db.clear();

	const setInitialManyData: [string, string][] = [];
	const size = 1_000;

	await new Promise<void>((done) => {
		for (let i = 0; i < size; i++) {
			setInitialManyData.push([`key-${i}`, randomData(100)]);

			spinner.update({
				text: `Setting up database (${i + 1}/${size}) (${(
					(i / size) *
					100
				).toFixed(2)}%)`,
			});
		}

		done();
	});

	spinner.update({
		text: "Setting up database",
	});

	await db.setMany(setInitialManyData);

	const setManyData: [string, string][] = Array.from(
		{ length: 100 },
		(_, i) => [`key-many-${i}`, randomData(100)],
	);

	bench
		.add("get", async () => {
			const res = await db.get(`key-${Math.floor(Math.random() * size)}`);

			if (res == null) {
				throw new Error("Value not found");
			}
		})
		.add("set", async () => {
			await db.set(`key-${Math.floor(Math.random() * size)}`, randomData(100));
		})
		.add("count", async () => {
			await db.count();
		})
		.add("setMany", async () => {
			await db.setMany(setManyData);
		});

	spinner.update({
		text: "Running warmup",
	});

	await bench.warmup();

	spinner.update({
		text: "Running benchmarks",
	});

	await bench.run();
	await db.close();

	spinner.stop();

	// clear line
	process.stdout.moveCursor(0, -1);
	process.stdout.clearLine(1);

	const table = bench.table();
	const finalTable: Record<string, string | number>[] = [];

	console.log(`😃 Results for: ${name}\n`); // Add Average Time (ms) column based on "Average Time (ns)" column

	for (const row of table) {
		if (!row) continue;
		if (typeof row["Average Time (ns)"] !== "number") continue;

		finalTable.push({
			...row,
			"Average Time (ms)": (row["Average Time (ns)"] / 1000000).toFixed(3),
		});
	}

	console.table(finalTable);
};

(async () => {
	const login = {
		user: "root",
		password: "root",
		database: "brinedb",
	};

	const { default: Enmap } = await import("enmap");

	const sqlite_memory = new Brine(BrineDatabases.sqlite.memory);
	const sqlite_file = new Brine(BrineDatabases.sqlite.file("benchmark.sqlite"));
	const enmap = new Enmap({
		name: "benchmark",
	});
	const postgres = new Brine(BrineDatabases.postgres.build(login));
	const mysql = new Brine(BrineDatabases.mysql.build(login));
	const mariadb = new Brine(
		BrineDatabases.mysql.build({
			...login,
			port: 3307,
		}),
	);

	await benchme("Brine SQLite (Memory)", sqlite_memory);
	await benchme("Brine SQLite (File)", sqlite_file);
	await benchme("Enmap SQLite (File)", {
		init: async () => {
			await enmap.defer;
		},
		clear: async () => enmap.clear(),
		set: async (key, value) => enmap.set(key, value),
		get: async (key) => enmap.get(key),
		setMany: async (data) => {
			for (const [key, value] of data) {
				enmap.set(key, value);
			}
		},
		count: async () => enmap.size,
		close: async () => {},
	});
	await benchme("PostgreSQL", postgres);
	await benchme("MySQL", mysql);
	await benchme("MariaDB", mariadb);

	console.log("✅ All  benchmarks complete");
})().catch(console.error);
