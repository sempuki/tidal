// tidal-rules: TPL-36
// tidal-expect: accept
// tidal-note: The member template's constant pack takes its types from the pack of the class template.
template <class... T> struct Outer {
  template <T... V> struct Inner { static constexpr int count = sizeof...(V); };
};
static_assert(Outer<int, char>::Inner<1, 'a'>::count == 2);
