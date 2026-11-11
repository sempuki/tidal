// tidal-rules: LSUP-1
// tidal-expect: reject
// tidal-error-clang: std::initializer_list was not found
// tidal-error-gcc: requires .#include <initializer_list>
// tidal-error-edg: #include <initializer_list> is needed
// tidal-note: Roadmap round 13, programmer probe range_for_braced_list_without_initializer_list.cpp (from intrusive_first_try.cpp). The range of the loop is a braced list, and the range variable deduces std::initializer_list<int> from it. The program declares no std::initializer_list (LSUP-1).
int main() {
  int a = 1, b = 2;
  int sum = 0;
  for (int v : {a, b}) sum += v;
  return sum - 3;
}
