// tidal-rules: CNS-11 CPY-1
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error-edg: static assertion failed
// tidal-error-clang-18: the constraint is checked
// tidal-note: W20-3, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-w20-3-constraint-checked.json. Q-0426. The copy of a const Ticket lvalue and the move of a Ticket xvalue each find a constructor that takes the argument with no conversion (CPY-1), so the constraint of the constructor template Ticket(const T&) is not checked (CNS-11). GCC 14 and 16 and Clang 22 run the program. EDG and Clang 18 check the constraint with T = Ticket, and the static assertion in Gate<Ticket> fails.
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
  Ticket(const Ticket&) = default;
  Ticket(Ticket&&) = default;
  template <class T> requires Gate<T>::open Ticket(const T&) : seat(0) {}
};
int main() {
  const Ticket a;
  Ticket b(a);
  Ticket c(static_cast<Ticket&&>(b));
  return c.seat == 4 ? 0 : 1;
}
