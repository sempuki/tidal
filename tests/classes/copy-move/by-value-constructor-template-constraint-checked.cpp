// tidal-rules: CPY-15 CNS-11
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: the constraint is checked for Ticket
// tidal-error-edg: static assertion failed
// tidal-note: W20-3, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-w20-3-constraint-checked.json. Q-0426. The copy of a Ticket lvalue that is not const finds the copy constructor, which adds const, so CNS-11 does not skip the constraint. The constraint of the constructor template Ticket(T) with T = Ticket is checked before the specialization is dropped (CPY-15), and the static assertion in Gate<Ticket> fails. Clang 18 and 22 and EDG reject. GCC 14 and 16 drop the specialization without the check and accept.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Ticket;
template <class T> struct Gate {
  static_assert(!same<T, Ticket>, "the constraint is checked for Ticket");
  static constexpr bool open = true;
};
struct Ticket {
  int seat = 4;
  Ticket() = default;
  template <class T> requires Gate<T>::open Ticket(T) : seat(0) {}
};
int main() {
  Ticket a;
  Ticket b(a);
  return b.seat == 4 ? 0 : 1;
}
