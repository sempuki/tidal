// Class template argument deduction with braces, return Box{k}, inside a
// function template. From views.cpp and expected.cpp.
template <class F> struct Box { F f; };
template <class K> constexpr auto times(K k) {
  return Box{k};
}
int main() { return times(2).f == 2 ? 0 : 1; }
