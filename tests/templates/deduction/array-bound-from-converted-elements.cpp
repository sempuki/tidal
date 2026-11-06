// tidal-rules: DED-1 DCL-5
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction unknown_bound_converted_elements_deduce_bound.cpp. A function template deduces the bound of an array whose bound came from a list of elements made by a converting constructor.
using size = decltype(sizeof 0);
template <class T, size N> constexpr size extent_of(T (&)[N]) { return N; }
struct Name {
  const char* text;
  Name(const char* t) : text(t) {}
};
int main() {
  Name names[] = {"ebb", "flow", "slack", "neap"};
  return extent_of(names) == 4 ? 0 : 1;
}
