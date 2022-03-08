# ⏱️ chrono-parse-rfc3339
RFC3339 Date to Microsecond Timestamp wasm module based on Rust Chrono

### 🥇 Install
```bash
npm install @qxip/chrono-parse-rfc3339
```

### Functions
- `parse_rfc3339` to nanoseconds
- `parse_nanos` to rfc3339
- `parse_micros` to rfc3339


### 🥈 Use
```javascript
const { parse_rfc3339, parse_nanos } = require("@qxip/chrono-parse-rfc3339");

const rfc3339 = "2018-12-18T08:28:06.801064-04:00"; // = 1545136086801064
const s = parse_rfc3339(rfc3339);
console.log(s); // 1545136086801064

const reverse = parse_nanos(s);
console.log(reverse); // = 2018-12-18T12:28:06.801064+00:00

```

### 🏗️ Build
Rebuild wasm package release. Not needed for regular module usage.
```bash
npm install
npm run build
npm test
```
