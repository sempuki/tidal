// tidal-rules: UNW-2
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}
int alive = 0;
struct Item {
  int v;
  explicit Item(int x) : v(x) { if (x == 3) throw x; ++alive; }
  ~Item() { --alive; }
};
int fill(Item* first, int n) {         // builds n Items one at a time
  int built = 0;
  try {
    for (; built != n; ++built) new (first + built) Item(built);
  } catch (...) {
    int left_by_language = alive;      // the language destroyed none of them
    while (built > 0) first[--built].~Item();
    ::operator delete(first);
    return left_by_language;
  }
  return -1;
}
int main() {
  Item* first = static_cast<Item*>(::operator new(sizeof(Item) * 5));
  int left = fill(first, 5);
  return left == 3 && alive == 0 ? 0 : 1;
}
