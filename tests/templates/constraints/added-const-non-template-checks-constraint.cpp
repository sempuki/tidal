// tidal-rules: CNS-11 SA-1
// tidal-expect: reject
// tidal-error-clang: the constraint is checked for Card
// tidal-error-gcc: the constraint is checked for Card
// tidal-error-edg: static assertion failed
// tidal-note: W20-3, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-w20-3-function-exact-match.json. The function show(const Card&) adds const to the Card lvalue, so it does not take the argument with no conversion (CNS-11). The constraint of the template show(const T&) with T = Card is checked, and the static assertion in Gate<Card> fails (SA-1). Every compiler rejects.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Card { int rank = 0; };
template <class T> struct Gate {
  static_assert(!same<T, Card>, "the constraint is checked for Card");
  static constexpr bool open = true;
};
int show(const Card&) { return 1; }
template <class T> requires Gate<T>::open int show(const T&) { return 2; }
int main() {
  Card c{};
  return show(c) == 1 ? 0 : 1;
}
