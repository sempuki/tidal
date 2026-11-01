// tidal-rules: ALN-8
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: non-integral type
// tidal-note: Q-0314. The operand has a class type with a constexpr conversion to int. Clang and EDG convert it and accept. GCC rejects it.
struct Eight { constexpr operator int() const { return 8; } };
constexpr Eight eight{};
struct alignas(eight) Block { int v; };
static_assert(alignof(Block) == 8);
