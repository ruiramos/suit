import { build } from "esbuild";
import watPlugin from "esbuild-plugin-wat";

build({
  entryPoints: ["src/app.jsx"],
  bundle: true,
  outfile: "dist/bundle.js",
  sourcemap: true,
  watch: {
    onRebuild(error, result) {
      if (error) console.error("watch build failed:", error);
      else console.log("watch build succeeded:", result);
    },
  },
  plugins: [watPlugin()],
});
