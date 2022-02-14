/*
import("../pkg/chrono_parse_rfc3339.js")
  .then((module) => {
    const rfc3339 = "2018-12-18T08:28:06.801064-04:00"; // 1545136086801064
    const s = module.parse_rfc3339(rfc3339);
    console.log(s);
  })
  .catch(console.error);
*/

const { parse_rfc3339 } = require("../pkg/chrono_parse_rfc3339.js");
const rfc3339 = "2018-12-18T08:28:06.801064-04:00"; // 1545136086801064
const s = parse_rfc3339(rfc3339);
console.log(s);

