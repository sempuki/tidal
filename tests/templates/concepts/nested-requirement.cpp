// tidal-rules: CPT-35
// tidal-expect: accept
// tidal-note: In Short, sizeof(T) > 2 is false for char, so T::value is never substituted.
template <class T> concept Wide = requires (T t) {
  t + 1;
  requires sizeof(t) > 2;
};
static_assert(Wide<int> && !Wide<short>);
template <class T> concept Short = requires { requires sizeof(T) > 2 && T::value; };
static_assert(!Short<char>);
template <class T> concept Never = requires { requires false; };
static_assert(!Never<int>);
