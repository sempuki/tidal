// tidal-rules: ODR-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0130. EDG accepts this use of a constant of the enclosing function.
// tidal-error-clang: cannot be implicitly captured
// tidal-error-gcc: .settings. is not captured
struct Settings { mutable int hits; };
int main() {
  constexpr Settings settings{4};
  auto read = [] { return settings.hits; };
  return read() - 4;
}
