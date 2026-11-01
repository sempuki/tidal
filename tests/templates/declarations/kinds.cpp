// tidal-rules: TPL-1
// tidal-expect: run
// tidal-note: One template of each kind: a class, a function, a variable, an alias, and a member function template.
template <class T> struct Box { T value; };
template <class T> T twice(T v) { return v + v; }
template <class T> constexpr T zero = T();
template <class T> using Pointer = T*;
struct Tool {
  template <class T> T pick(T a) { return a; }
};
int main() {
  Box<int> b{4};
  Pointer<int> p = &b.value;
  Tool t;
  return (twice<int>(*p) == 8 && zero<long> == 0 && t.pick<int>(3) == 3) ? 0 : 1;
}
