// tidal-rules: BF-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: has non-integral type
// tidal-note: Q-0314. The width has a class type with a constexpr conversion to int. Clang and EDG convert it and accept. GCC rejects it.
struct Three { constexpr operator int() const { return 3; } };
constexpr Three three{};
struct Flags { int mode : three; };
