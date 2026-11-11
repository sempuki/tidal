// A fixed-capacity vector over an array member of an anonymous union.
// T has no default constructor. Constant evaluation builds elements one
// at a time and sums them through a pointer and a length.
#include "construct.h"
struct Num {
  int v;
  constexpr Num(int x) : v(x) {}
  constexpr ~Num() {}
};
template <class T, int N>
class FixedVec {
  union { T items[N]; };
  int count = 0;
public:
  constexpr FixedVec() {}
  constexpr ~FixedVec() { while (count > 0) items[--count].~T(); }
  FixedVec(const FixedVec&) = delete;
  template <class... A>
  constexpr T* emplace(A&&... a) {
    T* p = std::construct_at(items + count, static_cast<A&&>(a)...);
    ++count;
    return p;
  }
  constexpr T* data() { return items; }
  constexpr int size() const { return count; }
};
constexpr int sum() {
  FixedVec<Num, 4> v;
  v.emplace(1); v.emplace(2); v.emplace(3);
  int s = 0;
  for (Num* p = v.data(); p != v.data() + v.size(); ++p) s += p->v;
  return s;
}
#ifdef RUN
int main() { return sum() == 6 ? 0 : 1; }
#else
static_assert(sum() == 6);
int main() {}
#endif
