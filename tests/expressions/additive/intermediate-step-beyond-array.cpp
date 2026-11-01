// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot refer to element .* of array of
// tidal-error-edg: cannot access position .* in array of
// tidal-note: Q-0031. The first step of p + n - n leaves the array, although the final position is inside it. GCC accepts the out-of-range pointer during constant evaluation; Clang and EDG reject it.
constexpr bool back_to_start() {
  int values[4] = {};
  int* p = values + 'a' - 'a';
  return p == values;
}
static_assert(back_to_start());
