// tidal-rules: NEW-36
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-note: Q-0302. The allocation function is a non-throwing member of the class. GCC throws for a bad bound also when the allocation function is non-throwing.
using size = decltype(sizeof 0);
alignas(16) unsigned char pool[256];
struct Item {
  int value = 1;
  static void* operator new[](size n) noexcept { return n > sizeof pool ? nullptr : pool; }
  static void operator delete[](void*) noexcept {}
};
long bound(long n) { return n; }
int main() {
  try {
    Item* p = new Item[bound(-1)];
    return p == nullptr ? 0 : 1;
  } catch (...) {
    return 2;
  }
}
