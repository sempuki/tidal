// tidal-rules: DED-17
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction constructor_template_array_reference_braced.cpp and probes function_template_array_reference_braced.cpp, constructor_template_array_reference_named.cpp, and constructor_array_reference_fixed_bound_braced.cpp (from intervals_second_try.cpp). The constructor template of Ranges takes a reference to an array of Span2 whose bound N it deduces. For a braced list of braced pairs, each pair is an argument for Span2, and N takes the number of pairs (DED-17).
struct Span2 { int from, to; };
struct Ranges {
  int count = 0;
  int last_to = 0;
  template <decltype(sizeof 0) N>
  constexpr Ranges(const Span2 (&list)[N]) : count(int(N)), last_to(list[N - 1].to) {}
};
static_assert(Ranges({{1, 2}, {3, 4}}).count == 2);
constexpr Ranges three({{0, 1}, {2, 3}, {4, 9}});
static_assert(three.count == 3 && three.last_to == 9);
int main() {
  Ranges one({{5, 6}});
  return one.count == 1 && one.last_to == 6 ? 0 : 1;
}
