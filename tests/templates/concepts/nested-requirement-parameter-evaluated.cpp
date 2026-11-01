// tidal-rules: CPT-35 CNS-9
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: non-constant expression
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: atomic constraint not constant
// tidal-note: t has no value, so t.value == 0 is not a constant expression. Clang 18 gives false instead (Q-0289).
struct Reading { int value; };
template <class T> concept Zero = requires (T t) { requires t.value == 0; };
bool zero = Zero<Reading>;
