// tidal-rules: EXS-4
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-note: Q-0094. GCC treats a class-specific operator delete without a specifier as potentially throwing. Clang and EDG treat it as non-throwing.
using Size = decltype(sizeof 0);
struct S { static void operator delete(void*, Size); };
static_assert(noexcept(S::operator delete(nullptr, 1)));
