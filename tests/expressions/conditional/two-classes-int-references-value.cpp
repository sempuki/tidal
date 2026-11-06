// tidal-rules: COND-11 OVL-11
// tidal-expect: run
// tidal-note: Decision 0059, item F2 (I4-10), I4 program p3b-two-int-refs-value.cpp. Two classes convert to int&. The result is an int prvalue with the value of the selected operand's referenced object.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
int first = 3, second = 4;
struct A { operator int&() { return first; } };
struct B { operator int&() { return second; } };
int main(int argc, char**) {
  A a;
  B b;
  static_assert(same<decltype(argc > 0 ? a : b), int>);
  int x = argc > 0 ? a : b;
  int y = argc < 0 ? a : b;
  return x == 3 && y == 4 ? 0 : 1;
}
