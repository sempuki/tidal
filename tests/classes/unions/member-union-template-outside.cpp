// tidal-rules: UNI-17
// tidal-expect: accept
// tidal-note: Front-end ledger row G182. A member union template is defined outside its class template, and a member union of a class template has an explicit specialization for one argument.
template <class T> struct Holder {
  template <class U> union Pair;
  union Slot { T value; };
};
template <class T> template <class U> union Holder<T>::Pair { T first; U second; };
template <> union Holder<int>::Slot { int value; long long wide; };
constexpr Holder<int>::Pair<char> pair{5};
static_assert(pair.first == 5);
static_assert(sizeof(Holder<char>::Pair<double>) == sizeof(double));
static_assert(sizeof(Holder<int>::Slot) == sizeof(long long));
static_assert(sizeof(Holder<char>::Slot) == sizeof(char));
