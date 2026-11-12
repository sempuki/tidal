// tidal-rules: PACK-14 PACK-5 TARG-17
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reductions template_template_pack_base_expansion.cpp and template_template_pack_as_arguments.cpp (from strong.cpp and its tries). Part is a pack of template template parameters. The base list Part<int>... gives one base for each template, in order (PACK-14, PACK-5). Forward passes the pack on as the argument list of Count, where each element is a template argument for a template template parameter (TARG-17).
template <class T> struct Width { int width = 3; };
template <class T> struct Height { int height = 4; };
template <template <class> class... Part> struct Shape : Part<int>... {};
template <template <class> class... Part> struct Count { static constexpr int n = sizeof...(Part); };
template <template <class> class... Part> struct Forward { static constexpr int n = Count<Part...>::n; };
static_assert(Forward<Width, Height>::n == 2 && Forward<>::n == 0);
int main() {
  Shape<Width, Height> s;
  Shape<Height> h;
  return s.width * s.height == 12 && h.height == 4 ? 0 : 1;
}
