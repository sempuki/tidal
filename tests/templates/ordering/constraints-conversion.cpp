// tidal-rules: ORD-13
// tidal-expect: accept
// tidal-note: Both conversion function templates have the result T*, so ordering ties and the constraint decides.
int a = 0;
int b = 0;
struct Source {
  template <class T> constexpr operator T*() const { return &a; }
  template <class T> requires (sizeof(T) > 1) constexpr operator T*() const { return &b; }
};
constexpr int* p = Source{};
static_assert(p == &b);
