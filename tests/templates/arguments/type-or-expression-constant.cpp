// tidal-rules: TARG-7 TARG-8
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: template argument for non-type template parameter is
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-note: int() reads as a type, so the constant parameter gets a type. EDG reads int() as the value 0 (Q-0170).
template <int N> struct Num {};
Num<int()> n;
