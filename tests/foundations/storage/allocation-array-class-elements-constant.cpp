// tidal-rules: STG-28 LSUP-11
// tidal-expect: accept
// tidal-note: The allocation gives an Item[4] whose elements have not begun their lifetimes. Constant evaluation accepts arithmetic over the elements and placement new at each one (Q-0323).
#include "construct.h"
namespace std {
template <class T> struct allocator {
  constexpr allocator() noexcept {}
  constexpr T* allocate(decltype(sizeof 0) n) {
    return static_cast<T*>(::operator new(n * sizeof(T)));
  }
  constexpr void deallocate(T* p, decltype(sizeof 0)) { ::operator delete(p); }
};
}
struct Item {
  int value;
  constexpr explicit Item(int v) : value(v) {}
  constexpr ~Item() {}
};
constexpr int fill() {
  std::allocator<Item> source;
  Item* items = source.allocate(4);
  std::construct_at(items + 3, 40);
  for (int i = 0; i != 3; ++i) std::construct_at(items + i, i * 10);
  int sum = 0;
  for (Item* p = items; p != items + 4; ++p) sum += p->value;
  for (int i = 4; i != 0; --i) items[i - 1].~Item();
  source.deallocate(items, 4);
  return sum;
}
static_assert(fill() == 70);
