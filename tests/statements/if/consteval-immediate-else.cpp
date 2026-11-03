// tidal-rules: IF-24
// tidal-expect: reject
// tidal-error-clang: call to consteval function .twice. is not a constant expression
// tidal-error-gcc: call to consteval function .twice\(v\). is not a constant expression
// tidal-error-edg: call to consteval function .twice. did not produce a valid constant expression
// tidal-note: The call is in the else branch of if consteval, which runs only outside constant evaluation.
consteval int twice(int v) { return v * 2; }
constexpr int f(int v) {
  if consteval {
    return v;
  } else {
    return twice(v);
  }
}
