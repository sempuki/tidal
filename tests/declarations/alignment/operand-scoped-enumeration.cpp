// tidal-rules: ALN-8
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: non-integral type
// tidal-error-clang: requires integer constant
// tidal-note: Q-0313. GCC and Clang reject the enumerator, and EDG converts it. A scoped enumeration does not convert to an integer, so its enumerator is not a valid operand.
enum class Width { eight = 8 };
struct alignas(Width::eight) Block { int v; };
