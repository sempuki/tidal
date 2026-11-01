// tidal-rules: PACK-16
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: On a class declaration, GCC takes the alignment of the last element. Clang and EDG take the strictest (Q-0238).
struct alignas(2) Two { char c; };
struct alignas(8) Eight { char c; };
template <class... T> struct alignas(T...) Strict { char c; };
static_assert(alignof(Strict<Eight, Two>) == 8);
