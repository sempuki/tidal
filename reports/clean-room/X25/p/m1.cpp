// Migration of a span sort: P27's FixedVec hands out a pointer and a
// length over its elements, and the sort works on the elements. Runs
// today on both compilers; under P27 the reads through items are defined.
#include "construct.h"
struct Word {
  int key;
  constexpr Word(int k) : key(k) {}
  constexpr ~Word() {}
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
  constexpr T& emplace(A&&... a) {
    T* p = std::construct_at(items + count, static_cast<A&&>(a)...);
    ++count;
    return *p;
  }
  constexpr T* data() { return items; }
  constexpr int size() const { return count; }
};
template <class T>
constexpr void sort(T* first, int n) {        // insertion sort by swapping keys
  for (int i = 1; i < n; ++i)
    for (int j = i; j > 0 && first[j].key < first[j - 1].key; --j) {
      int k = first[j].key; first[j].key = first[j - 1].key; first[j - 1].key = k;
    }
}
constexpr int run() {
  FixedVec<Word, 8> v;
  v.emplace(3); v.emplace(1); v.emplace(2);
  sort(v.data(), v.size());
  return v.data()[0].key * 100 + v.data()[1].key * 10 + v.data()[2].key;
}
#ifdef RUN
int main() { return run() == 123 ? 0 : 1; }
#else
static_assert(run() == 123);
int main() {}
#endif
