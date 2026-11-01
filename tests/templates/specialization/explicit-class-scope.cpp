// tidal-rules: TSP-19
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-note: An explicit specialization of a member template inside the class that declares the template. GCC rejects an explicit specialization in a class (Q-0171).
struct Codes {
  template <class T> static constexpr int of() { return 0; }
  template <> constexpr int of<int>() { return 1; }
};
static_assert(Codes::of<int>() == 1);
static_assert(Codes::of<char>() == 0);
