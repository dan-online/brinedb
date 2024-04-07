import { Brine } from "../../";
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

const benchme = async (name: string, db: Brine) => {
	const spinner = new Spinner("Initializing database");
	const bench = new Bench({ time: 1000, warmupTime: 500 });

	spinner.start();

	await db.init();

	spinner.update({
		text: "Setting up database",
	});

	await db.clear();

	const setInitialManyData: [string, string][] = [];

	for (let i = 0; i < 1_000_000; i++) {
		setInitialManyData.push([`key-${i}`, randomData(100)]);

		spinner.update({
			text: `Setting up database (${i + 1}/1000000) (${(
				(i / 1_000_000) *
				100
			).toFixed(2)}%)`,
		});
	}

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
			await db.get(`key-${Math.floor(Math.random() * 1000)}`);
		})
		.add("set", async () => {
			await db.set(`key-${Math.floor(Math.random() * 1000)}`, randomData(100));
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

	const sqlite_memory = new Brine(BrineDatabases.sqlite.memory);
	const sqlite_file = new Brine(BrineDatabases.sqlite.file("benchmark.sqlite"));
	const postgres = new Brine(BrineDatabases.postgres.build(login));
	const mysql = new Brine(BrineDatabases.mysql.build(login));
	const mariadb = new Brine(
		BrineDatabases.mysql.build({
			...login,
			port: 3307,
		}),
	);

	await benchme("SQLite (Memory)", sqlite_memory);
	await benchme("SQLite (File)", sqlite_file);
	await benchme("PostgreSQL", postgres);
	await benchme("MySQL", mysql);
	await benchme("MariaDB", mariadb);

	console.log("✅ All  benchmarks complete");
})().catch(console.error);
