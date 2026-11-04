// tidal-rules: ODR-2 ODR-3
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-error-clang: cannot be implicitly captured
// tidal-note: Q-0367. The value of the expression origin is discarded, so the name uses nothing. GCC and EDG accept. Clang rejects.
struct Point { int x; };
int main() {
  constexpr Point origin{0};
  auto touch = [] { origin; return 0; };
  return touch();
}
