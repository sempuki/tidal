// tidal-rules: OVL-18
// tidal-expect: accept
// tidal-note: Static members and members with an explicit object parameter match ordinary pointers. Members with an implicit object parameter match member pointers, also a member pointer of a derived class.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {
  One get(int);
  static Two get(long);
  One put(this Item&, int);
  Two put(long);
};
struct Derived : Item {};
One (Item::*a)(int) = &Item::get;
Two (*b)(long) = &Item::get;
One (*c)(Item&, int) = &Item::put;
Two (Item::*d)(long) = &Item::put;
One (Derived::*e)(int) = &Item::get;
