// tidal-rules: BF-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: has non-integral type
// tidal-error-clang: must have integral or unscoped enumeration type
// tidal-note: Q-0313. GCC and Clang reject the enumerator, and EDG converts it. A scoped enumeration does not convert to an integer, so its enumerator is not a valid width.
enum class Bits { three = 3 };
struct Flags { int mode : Bits::three; };
