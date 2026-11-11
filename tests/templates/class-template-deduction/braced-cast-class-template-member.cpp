// tidal-rules: CTAD-1 CTAD-9
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe ctad_braced_in_class_template_member.cpp. A functional cast with braces, Wrapper{v}, in a static member function of a class template deduces Wrapper<V> from the aggregate candidate (CTAD-1, CTAD-9).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class Item> struct Wrapper { Item item; };
template <class V> struct Factory {
  static constexpr auto build(V v) { return Wrapper{v}; }
};
static_assert(same<decltype(Factory<long>::build(5)), Wrapper<long>>);
int main() { return Factory<int>::build(6).item == 6 ? 0 : 1; }
