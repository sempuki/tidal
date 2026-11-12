// tidal-rules: DER-1
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction next_class_template_own_name_in_base.cpp, from the rewrite's runs (RW2-2). The base list of the class template Item names Counted<Item<T> >, a specialization whose argument is the class being defined. The base is a complete class, and it is not Item<T> itself, so DER-1 accepts it. Plain does the same with a class that is not a template.
template <class Derived> struct Counted {
  int tag = 1;
  constexpr int twice() const { return static_cast<const Derived&>(*this).weight() * 2; }
};
template <class T> struct Item : Counted<Item<T> > {
  T w = 3;
  constexpr int weight() const { return int(w); }
};
struct Plain : Counted<Plain> { constexpr int weight() const { return 5; } };
static_assert(Item<int>{}.twice() == 6 && Plain{}.twice() == 10);
int main() {
  Item<long> i;
  return i.tag + i.twice() == 7 ? 0 : 1;
}
