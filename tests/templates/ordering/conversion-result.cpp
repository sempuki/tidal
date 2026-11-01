// tidal-rules: ORD-8
// tidal-expect: accept
// tidal-note: For conversion function templates only the result types take part. T* is more specialized than T.
int first = 0;
int second = 0;
struct Source {
  template <class T> constexpr operator T() const { return &first; }
  template <class T> constexpr operator T*() const { return &second; }
};
constexpr int* p = Source{};
static_assert(p == &second);
