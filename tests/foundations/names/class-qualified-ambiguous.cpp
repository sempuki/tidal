// tidal-rules: NAM-18
// tidal-expect: reject
// tidal-error-clang: member .value. found in multiple base classes
// tidal-error-gcc: reference to .value. is ambiguous
// tidal-error-edg: join::value. is ambiguous
struct left { static constexpr int value = 1; };
struct right { static constexpr int value = 2; };
struct join : left, right { };
int read() { return join::value; }
