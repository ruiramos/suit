import { build } from "esbuild";
import watPlugin from "esbuild-plugin-wat";

build({
  entryPoints: ["src/app.jsx"],
  bundle: true,
  outfile: "dist/bundle.js",
  sourcemap: true,
  watch: true,
  plugins: [watPlugin()],
});
