// tidal-rules: CE-26
// tidal-expect: accept
// The values of these constants are undefined. The test checks only that
// the implementation can accept them.
constexpr int both_write() { int i = 1; return i++ + i++; }
constexpr int read_and_write() { int i = 1; return i + i++; }
constexpr int product() { int i = 1; return i++ * i; }
constexpr int a = both_write();
constexpr int b = read_and_write();
constexpr int c = product();
