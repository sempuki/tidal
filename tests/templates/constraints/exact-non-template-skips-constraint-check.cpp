// tidal-rules: CNS-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error-edg: static assertion failed
// tidal-error-clang-18: the constraint is checked
// tidal-note: W20-3, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-w20-3-function-exact-match.json. Q-0426. The function show(const Card&) takes the const Card lvalue with no conversion, so the constraint of the template show(const T&) is not checked, and Gate<Card> is never instantiated. GCC 14 and 16 and Clang 22 accept. EDG and Clang 18 check the constraint, and the static assertion in Gate<Card> fails.
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
  const Card c{};
  return show(c) == 1 ? 0 : 1;
}
