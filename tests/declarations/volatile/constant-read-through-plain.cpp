// tidal-rules: VOL-6 VOL-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-note: GCC and EDG accept the read of the volatile object through int* during constant evaluation (Q-0320).
// tidal-error-clang: read of volatile object
constexpr int read_through_plain() {
  volatile int level = 3;
  int* plain = const_cast<int*>(&level);
  return *plain;   // undefined: level is volatile
}
constexpr int value = read_through_plain();
int main() { return value == 3 ? 0 : 1; }
