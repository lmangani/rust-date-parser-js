# chrono-parse-rfc3339
RFC3339 Date to Microsecond Timestamp wasm module based on Rust Chrono

### Install
```
npm install @qxip/chrono-parse-rfc3339
```

### Use
```
const { parse_rfc3339 } = require("@qxip/chrono-parse-rfc3339");
const rfc3339 = "2018-12-18T08:28:06.801064-04:00"; // 1545136086801064
const s = parse_rfc3339(rfc3339);
console.log(s); // 1545136086801064
```
