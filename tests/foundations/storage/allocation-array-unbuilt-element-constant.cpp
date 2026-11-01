// tidal-rules: OBJ-2 STG-28
// tidal-expect: reject
// tidal-error-clang: read of object outside its lifetime
// tidal-error-gcc: accessing uninitialized array element
// tidal-error-edg: access to uninitialized object
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
constexpr int second() {
  std::allocator<Item> source;
  Item* items = source.allocate(2);
  std::construct_at(items, 1);
  int v = items[1].value;              // element 1 has not begun its lifetime
  items[0].~Item();
  source.deallocate(items, 2);
  return v;
}
static_assert(second() == 0);
