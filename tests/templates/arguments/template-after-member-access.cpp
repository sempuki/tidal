// tidal-rules: TARG-2
// tidal-expect: accept
// tidal-note: Member lookup of size_of after the dot finds a member function template, so < opens a template argument list.
struct Gauge {
  template <class T> constexpr int size_of(int extra) const { return int(sizeof(T)) + extra; }
};
static_assert(Gauge{}.size_of<char>(1) == 2);
