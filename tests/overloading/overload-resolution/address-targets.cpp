// tidal-rules: OVL-18
// tidal-expect: accept
// tidal-note: Each context gives a target type, and the target selects one function from the set.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One pick(int);
Two pick(long);
One (*p)(int) = pick;
Two (*q)(long) = &pick;
Two (&r)(long) = pick;
void take(One (*)(int));
void use() {
  take(pick);
  p = pick;
  (void)static_cast<Two (*)(long)>(pick);
}
One (*give())(int) { return pick; }
template <One (*F)(int)> struct Holder {};
Holder<pick> h;
