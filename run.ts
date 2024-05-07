import init, { parse_wat } from "./mod.ts";

await init();

const result = parse_wat("(module)");
console.log(result);