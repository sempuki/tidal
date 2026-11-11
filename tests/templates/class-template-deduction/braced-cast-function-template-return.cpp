// tidal-rules: CTAD-1 CTAD-9
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction ctad_braced_in_function_template_return.cpp. A functional cast with braces and no argument list, Wrapper{v}, is a site of deduction (CTAD-1), also in the return statement of a function template. The aggregate candidate (CTAD-9) gives Wrapper<V>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class Item> struct Wrapper { Item item; };
template <class V> constexpr auto wrap(V v) {
  return Wrapper{v};
}
static_assert(same<decltype(wrap(4)), Wrapper<int>>);
static_assert(wrap(4).item == 4);
int main() { return wrap('k').item == 'k' ? 0 : 1; }
