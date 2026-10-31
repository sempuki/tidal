// tidal-rules: ENUM-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: outside the valid range of values|must be initialized by a constant expression
// tidal-note: Q-0017. Clang rejects a value outside the enumeration's range during constant evaluation; GCC and EDG accept it.
enum E { A = 1, B = 4 };
constexpr E e = static_cast<E>(8);
