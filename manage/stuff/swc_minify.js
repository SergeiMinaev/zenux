import { transformSync } from 'npm:@swc/core';

const code = await Deno.readTextFile("tods.js");
const r = transformSync(code, { jsc: { target: 'es2022', minify: { compress: true, mangle: true } }, minify: true });
await Deno.writeTextFile("tods.swj.js", r.code);
