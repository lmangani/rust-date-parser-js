const { parse_rfc3339 } = require("../pkg/chrono_parse_rfc3339.js");

describe('chrono_parse_rfc3339', () => {
  test('convert date to microsecond ts', () => {
    const rfc3339 = "2018-12-18T08:28:06.801064-04:00"; // 1545136086801064
    const result = parse_rfc3339(rfc3339);
    expect(result).toEqual("1545136086801064");
  });
});
