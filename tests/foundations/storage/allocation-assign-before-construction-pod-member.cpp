// tidal-rules: LSUP-11 CE-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: assignment to object outside its lifetime
// tidal-note: Q-0421. Decision 0059, item QR, with the round 9 writer probe q-0323-assign-before-construction.json. In constant evaluation, the program assigns to a member of a Pod element of storage from std::allocator before std::construct_at builds it. Clang rejects; GCC and EDG accept.
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
struct Pod { int v; };
constexpr int probe() {
  std::allocator<Pod> source;
  Pod* p = source.allocate(1);
  p->v = 5;                            // no Pod has begun its lifetime here
  int v = p->v;
  source.deallocate(p, 1);
  return v;
}
static_assert(probe() == 5);
