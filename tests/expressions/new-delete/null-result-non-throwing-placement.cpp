// tidal-rules: NEW-31
// tidal-expect: run
using size = decltype(sizeof 0);
int built = 0;
void* operator new(size, int) noexcept { return nullptr; }
void* operator new[](size, int) noexcept { return nullptr; }
struct Gadget { int value = 5; Gadget() { built = 1; } };
int main() {
  Gadget* one = new (1) Gadget;
  Gadget* many = new (2) Gadget[3];
  static_assert(noexcept(new (1) int));
  return one == nullptr && many == nullptr && built == 0 ? 0 : 1;
}
