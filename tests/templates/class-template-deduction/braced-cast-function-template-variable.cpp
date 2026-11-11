// tidal-rules: CTAD-1 CTAD-9
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe ctad_braced_auto_variable_in_function_template.cpp. In a function template, an auto variable is initialized from a functional cast with braces, Wrapper{closure}. The cast deduces Wrapper with the closure type from the aggregate candidate (CTAD-1, CTAD-9).
template <class Item> struct Wrapper { Item item; };
template <class V> constexpr auto add_to(V v) {
  auto w = Wrapper{[v](int x) { return x + v; }};
  return w;
}
static_assert(add_to(3).item(4) == 7);
int main() { return add_to(10).item(5) == 15 ? 0 : 1; }
