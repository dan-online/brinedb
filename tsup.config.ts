import { defineConfig } from "tsup";

export default defineConfig({
	tsconfig: "./tsconfig.json",
	clean: true,
	entry: ["src/index.ts"],
	external: ["../native/brinedb.node"],
	treeshake: false,
	dts: true,
	format: ["cjs", "esm"],
});
