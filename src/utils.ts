export const BrineDatabases = {
	sqlite: {
		memory: "sqlite::memory:",
		file: (path: string) => `sqlite:${path}?mode=rwc`,
	},
	postgres: {
		build: (options: {
			user: string;
			password: string;
			host: string;
			port: number;
			database: string;
		}) => {
			const { user, password, host, port, database } = options;

			return `postgres://${user}:${password}@${host}:${port}/${database}`;
		},
	},
	mysql: {
		build: (options: {
			user: string;
			password: string;
			host: string;
			port: number;
			database: string;
		}) => {
			const { user, password, host, port, database } = options;

			return `mysql://${user}:${password}@${host}:${port}/${database}`;
		},
	},
};
