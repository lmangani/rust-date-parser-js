const { parse_rfc3339, parse_nanos, parse_micros } = require("../pkg/chrono_parse_rfc3339.js");

describe('chrono_parse_rfc3339', () => {
  test('convert rfc3339 to nanosecond ts', () => {
    const rfc3339 = "2018-12-18T08:28:06.801064-04:00"; // 1545136086801064
    const result = parse_rfc3339(rfc3339);
    expect(result).toEqual("1545136086801064");
  });
  test('convert nanosecond ts to rfc3339', () => {
    const rfc3339 = "2018-12-18T12:28:06.801064+00:00"; // 1545136086801064
    const result = parse_nanos("1545136086801064");
    expect(result).toEqual(rfc3339);
  });
  test('convert microsecond ts to rfc3339', () => {
    const rfc3339 = "2018-12-18T12:28:06.801+00:00"; // 1545136086801
    const result = parse_micros("1545136086801");
    expect(result).toEqual(rfc3339);
  });

});
