// tidal-rules: CAST-16
// tidal-expect: run
using size = decltype(sizeof 0);
static_assert(sizeof(size) == sizeof(void*) && sizeof(unsigned long) == sizeof(void*));
static_assert(sizeof(unsigned) < sizeof(void*));
int main() {
  int value = 7;
  size held = reinterpret_cast<size>(&value);
  return held % alignof(int) == 0 && *reinterpret_cast<int*>(held) == 7 ? 0 : 1;
}
