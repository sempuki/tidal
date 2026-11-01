// tidal-rules: DEP-20
// tidal-expect: run
struct Has {
  using type = int;
  struct base { int b = 1; };
};
template <class T> struct Box { T::type value; };
template <class T> struct Uses : T::base {
  using alias = T::type;
  T::type member = 2;
  T::type get() { return member; }
  void set(T::type v) { member = v; }
  T::type later();
  void store(T::type v);
  friend int touch(Uses, T::type v) { return v; }
};
template <class T> T::type Uses<T>::later() { return 3; }
template <class T> void Uses<T>::store(T::type v) { member = v; }
template <class T> T::type make() { return 4; }
template <class T> auto make_trailing() -> T::type { return 5; }
template <class T, class U = T::type> struct Defaulted { U value = 6; };
template <class T, T::type N = 7> struct Fixed { static constexpr int value = N; };
template <class T> T::type zero = 0;
template <class T> int body(long v) {
  auto p = new T::type(static_cast<T::type>(v));
  int r = *p;
  delete p;
  return r + [](T::type x) { return x; }(1);
}
int main() {
  Uses<Has> u;
  u.set(2);
  u.store(u.get());
  int sum = u.b + u.later() + touch(u, 0) + make<Has>() + make_trailing<Has>()
          + Defaulted<Has>{}.value + Fixed<Has>::value + zero<Has> + body<Has>(8);
  return sum == 1 + 3 + 0 + 4 + 5 + 6 + 7 + 0 + 9 ? 0 : 1;
}
