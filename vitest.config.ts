import { defineConfig } from "vitest/config";

export default defineConfig({
	optimizeDeps: {
		exclude: ["."],
	},
	test: {
		coverage: {
			provider: "v8",
		},
	},
});
