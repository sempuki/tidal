// tidal-rules: LSUP-11 CE-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: outside its lifetime
// tidal-note: Q-0421. G156 (front-end report 23), with the writer probe reports/clean-room/roadmap-10/writer-probes/g156-assign-before-construction-types.json. In constant evaluation, the program assigns to an element of a class with its own assignment operator, in storage from std::allocator, before std::construct_at builds it. Clang rejects; GCC and EDG accept, as for an int element.
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
struct Meter {
  int reading;
  constexpr Meter& operator=(int r) { reading = r; return *this; }
};
constexpr int probe() {
  std::allocator<Meter> source;
  Meter* p = source.allocate(2);
  p[1] = 7;                            // no Meter has begun its lifetime here
  int v = p[1].reading;
  source.deallocate(p, 2);
  return v;
}
static_assert(probe() == 7);
