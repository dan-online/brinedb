import { defineConfig } from "tsup";

export default defineConfig({
	tsconfig: "./tsconfig.json",
	clean: true,
	entryPoints: ["src/index.ts"],
	format: ["cjs"],
	// regex ends in .node
	external: [/\.node$/],
});
