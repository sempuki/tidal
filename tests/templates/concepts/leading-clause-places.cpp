// tidal-rules: CPT-8
// tidal-expect: accept
template <class T> concept Big = sizeof(T) > 1;
template <class T> requires Big<T> struct Box {};
template <class T> requires Big<T> constexpr int twice(T v) { return v + v; }
template <class T> requires Big<T> constexpr int one = 1;
template <class T> requires Big<T> using Same = T;
struct Holder {
  template <class T> requires Big<T> static constexpr int get(T) { return 2; }
};
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> requires Big<T> struct Kind<T*> { static constexpr int value = 1; };
Box<int> box;
Same<int> same = 0;
static_assert(twice(2) == 4 && one<int> == 1 && Holder::get(1) == 2);
static_assert(Kind<int*>::value == 1 && Kind<char*>::value == 0);
