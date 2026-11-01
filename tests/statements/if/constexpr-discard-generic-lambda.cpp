// tidal-rules: IF-13
// tidal-expect: accept
// tidal-note: The body of a generic lambda at namespace scope and a member of a local class in a function template are templated contexts. Their discarded statements are not instantiated.
auto pick = [](auto x) {
  if constexpr (sizeof(x) > 100) { x.missing(); return 1; }
  else return 0;
};
int y = pick(1);
template <class T> int local() {
  struct Inner {
    int run() {
      if constexpr (sizeof(T) > 100) { return T::missing(); }
      else return 2;
    }
  };
  return Inner{}.run();
}
int z = local<int>();
