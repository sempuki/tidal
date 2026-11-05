// tidal-rules: CPY-28 STG-12
// tidal-expect: run
// tidal-note: A scoped enumeration, a floating value, a pointer to member function, a value of the null pointer type, and a const array of arrays of a class keep their values through a byte copy.
struct Pair { double d; long n; };
struct Holder { int f() const { return 7; } };
enum class Level : short { low = 1, high = 9 };
template <class T> void copy_bytes(const T& from, T& to) {
  const unsigned char* src = reinterpret_cast<const unsigned char*>(&from);
  unsigned char* dst = reinterpret_cast<unsigned char*>(&to);
  for (unsigned i = 0; i < sizeof(T); ++i) dst[i] = src[i];
}
int main() {
  Level level = Level::high, level_copy = Level::low;
  copy_bytes(level, level_copy);
  double real = 2.5, real_copy = 0.0;
  copy_bytes(real, real_copy);
  int (Holder::* call)() const = &Holder::f;
  int (Holder::* call_copy)() const = nullptr;
  copy_bytes(call, call_copy);
  decltype(nullptr) null = nullptr, null_copy = nullptr;
  copy_bytes(null, null_copy);
  const Pair grid[2][2] = {{{1.5, 1}, {2.5, 2}}, {{3.5, 3}, {4.5, 4}}};
  Pair grid_copy[2][2] = {};
  copy_bytes(grid, grid_copy);
  Holder h;
  bool ok = level_copy == Level::high && real_copy == 2.5 && (h.*call_copy)() == 7
            && null_copy == nullptr && grid_copy[1][0].d == 3.5 && grid_copy[1][1].n == 4;
  return ok ? 0 : 1;
}
