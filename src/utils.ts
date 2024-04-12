export const BrineDatabases = {
	sqlite: {
		/**
		 * In-memory SQLite database
		 */
		memory: "sqlite://:memory:",
		/**
		 * SQLite database stored on disk
		 *
		 * @param path
		 * @returns The connection URI
		 */
		file: (path: string) => `sqlite://${path}`,
	},
	postgres: {
		/**
		 * Build a connection URI for a PostgreSQL database
		 * @param options
		 * @returns
		 */
		build: (options: {
			user: string;
			password: string;
			host?: string;
			port?: number;
			database: string;
		}) => {
			const defaultOptions = {
				host: "localhost",
				port: 5432,
			};

			const { user, password, host, port, database } = {
				...defaultOptions,
				...options,
			};

			return `postgres://${user}:${password}@${host}:${port}/${database}`;
		},
	},
	mysql: {
		/**
		 * Build a connection URI for a MySQL database
		 * @param options
		 * @returns
		 */
		build: (options: {
			user: string;
			password: string;
			host?: string;
			port?: number;
			database: string;
		}) => {
			const defaultOptions = {
				host: "localhost",
				port: 3306,
			};

			const { user, password, host, port, database } = {
				...defaultOptions,
				...options,
			};

			return `mysql://${user}:${password}@${host}:${port}/${database}`;
		},
	},
};
