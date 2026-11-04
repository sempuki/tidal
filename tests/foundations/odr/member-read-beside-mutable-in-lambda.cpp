// tidal-rules: ODR-2 ODR-3
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: cannot be implicitly captured
// tidal-error-gcc: .settings. is not captured
// tidal-note: Q-0130. size is not mutable, but Settings has a mutable member, so the read uses settings. EDG accepts.
struct Settings { int size; mutable int hits; };
int main() {
  constexpr Settings settings{4, 0};
  auto read = [] { return settings.size; };
  return read() - 4;
}
