// tidal-rules: TSP-3
// tidal-expect: accept
// tidal-note: A member template of a specialization of a class template gets an explicit specialization.
template <class T> struct Box {
  template <class U> static constexpr int pick() { return 1; }
};
template <> template <> constexpr int Box<int>::pick<char>() { return 2; }
static_assert(Box<int>::pick<char>() == 2);
static_assert(Box<int>::pick<long>() == 1);
static_assert(Box<long>::pick<char>() == 1);
