// tidal-rules: ODR-2 ODR-3
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: .local. is not captured
// tidal-error-edg: cannot be referenced in a lambda body unless it is in the capture list
// tidal-note: Q-0367. The branch is discarded, so &local needs no definition, but the use still counts for the capture rule. GCC and EDG reject. Clang accepts.
int main() {
  int local = 1;
  auto pick = [] {
    if constexpr (false) { return &local; } else { return static_cast<int*>(nullptr); }
  };
  return pick() == nullptr ? 0 : 1;
}
