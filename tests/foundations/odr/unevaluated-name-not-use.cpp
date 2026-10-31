// tidal-rules: ODR-3
// tidal-expect: run
int declared_only();
extern int also_declared_only;
int main() {
  decltype(declared_only()) copy = 1;
  bool may_throw = !noexcept(declared_only());
  if constexpr (sizeof(also_declared_only) == 0) {
    return declared_only();
  }
  return sizeof(declared_only()) == sizeof(int) && may_throw && copy == 1 ? 0 : 1;
}
