// tidal-rules: ODR-2 ODR-3
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-error-clang: cannot be implicitly captured
// tidal-note: Q-0367. The value of the expression n is discarded, so the name uses nothing. GCC and EDG accept. Clang rejects.
int main() {
  const int n = 5;
  auto touch = [] { n; return 0; };
  return touch();
}
