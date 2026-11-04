// tidal-rules: TR-3
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: cannot be declared in a linkage-specification
// tidal-note: Q-0366. Clang and GCC accept main declared inside an extern "C++" block. EDG rejects it.
extern "C++" {
  int main() { return 0; }
}
