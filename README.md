# wasm-llm-parser

Experimental Rust/WebAssembly helpers for extracting simple fenced code blocks, thought-tag text, and tracking bracket balance across streamed text chunks.

```sh
cargo test
wasm-pack build --target web
```

For a web-targeted build, initialize the generated module before using it:

```js
import init, { StreamJsonValidator, extract_code_blocks } from './pkg/wasm_llm_parser.js';
await init();
const stream = new StreamJsonValidator();
stream.feed_chunk('{"items": [1,');
stream.feed_chunk('2]}');
console.log(stream.is_balanced());
stream.free();
console.log(extract_code_blocks('```js\nconsole.log(1)\n```'));
```

`StreamJsonValidator` checks nesting and quotes only; **balanced brackets do not mean valid JSON**. It does not validate commas, keys, values, escape sequences or UTF-8 byte boundaries. Validate the completed document with a JSON parser before accepting it. An unmatched closing bracket or incorrect nesting permanently marks the stream mismatched. Markdown and tag helpers implement simple extraction, not the full CommonMark or XML specifications. Inspect and sanitize output appropriately before rendering or executing it.

The Rust tests and a local web-targeted WASM build pass; browser and deployment compatibility must be verified in the target environment.

[Apache License 2.0](LICENSE).
