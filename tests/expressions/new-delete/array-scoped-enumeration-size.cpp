// tidal-rules: NEW-17 NEW-2
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-error-clang: incompatible type
// tidal-error-edg: must have integral or unscoped enum type
// tidal-note: Q-0313. A scoped enumeration does not convert to the type of sizeof. Clang and EDG reject the size. GCC converts it.
enum class Count { two = 2 };
int main() {
  int* p = new int[Count::two];
  p[1] = 0;
  delete[] p;
  return 0;
}
