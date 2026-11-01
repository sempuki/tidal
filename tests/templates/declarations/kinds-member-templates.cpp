// tidal-rules: TPL-1
// tidal-expect: accept
// tidal-note: A class has a member class template, a static data member template, a member alias template, and a member function template.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct Tool {
  template <class T> struct Inner { T value; };
  template <class T> static constexpr int size = sizeof(T);
  template <class T> using Pointer = T*;
  template <class T> static constexpr T keep(T v) { return v; }
};
static_assert(Tool::Inner<int>{3}.value == 3);
static_assert(Tool::size<char> == 1);
static_assert(same<Tool::Pointer<int>, int*>);
static_assert(Tool::keep(4) == 4);
