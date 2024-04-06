import { Brine } from "../../dist";
import Josh from "@joshdb/core";
// @ts-expect-error 7016
import JoshSqlite from "@joshdb/sqlite";
import { Spinner } from "@favware/colorette-spinner";
import { BrineDatabases } from "../../src";

const runs = 100;
const databaseSize = 5;

console.log(
	`\nBrine Benchmark commencing, runs set at ${runs} and ${databaseSize} database size.\n`,
);

interface kv {
	set: (key: string, value: string) => Promise<void>;
	get: (key: string) => Promise<void>;
	clear: () => Promise<void>;
}

const test = async (store: kv) => {
	const start = performance.now();

	const randomKey = Math.random().toString(36).substring(7);
	const randomValue = Math.random().toString(36).substring(7);

	await store.set(randomKey, randomValue);
	console.log(await store.get(randomKey));

	const end = performance.now();

	return end - start;
};

const spinner = new Spinner("Benchmark");
const benchmark = async (name: string, runner: kv) => {
	spinner.start({
		text: `Benchmarking (${name}): 0/${runs}`,
	});

	let total = 0;
	let lastUpdate = Date.now();

	await runner.clear();

	for (let i = 0; i < runs; i++) {
		total += await test(runner);
		spinner.update({
			text: `Benchmarking (${name}): ${i + 1}/${runs}`,
		});

		if (lastUpdate + 50 < Date.now()) {
			lastUpdate = Date.now();
			spinner.spin();
		}
	}

	return {
		[name]: {
			"Average (ms)": (total / runs).toFixed(3),
			"Operations (op/s)": Math.round(runs / (total / 1000)).toLocaleString(),
			"Total (s)": (total / 1000).toFixed(2),
		},
	};
};

(async () => {
	const josh = new Josh({
		name: "josh",
		provider: JoshSqlite,
	});

	const joshResults = await benchmark("Josh (SQLite)", {
		set: async (key, value) => {
			await josh.set(key, value);
		},
		get: async (key) => {
			await josh.get(key);
		},
		clear: async () => {
			await josh.delete(josh.all);
		},
	});

	const brine_postgres = new Brine<string>(
		BrineDatabases.postgres.build({
			host: "localhost",
			port: 5432,
			user: "root",
			password: "root",
			database: "brine",
		}),
	);

	await brine_postgres.init();

	const brineResults = await benchmark("Brine (Postgres)", {
		set: async (key, value) => {
			await brine_postgres.set(key, value);
		},
		get: async (key) => {
			await brine_postgres.get(key);
		},
		clear: async () => {
			await brine_postgres.clear();
		},
	});

	const brine_sqlite = new Brine<string>(
		BrineDatabases.sqlite.file("./data/brine.sqlite"),
	);

	await brine_sqlite.init();

	const brineResultsSqlite = await benchmark("Brine (SQLite)", {
		set: async (key, value) => {
			await brine_sqlite.set(key, value);
		},
		get: async (key) => {
			await brine_sqlite.get(key);
		},
		clear: async () => {
			await brine_sqlite.clear();
		},
	});

	const brine_memory = new Brine<string>(BrineDatabases.sqlite.memory);

	await brine_memory.init();

	const brineResultsMemory = await benchmark("Brine (Memory)", {
		set: async (key, value) => {
			await brine_memory.set(key, value);
		},
		get: async (key) => {
			await brine_memory.get(key);
		},
		clear: async () => {
			await brine_memory.clear();
		},
	});

	spinner.success({ text: "Benchmarking complete!" });
	console.table({
		...joshResults,
		...brineResults,
		...brineResultsSqlite,
		...brineResultsMemory,
	});
})();
