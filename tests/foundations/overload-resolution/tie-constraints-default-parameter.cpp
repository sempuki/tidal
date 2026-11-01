// tidal-rules: OVL-30 OVL-36
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int\). is ambiguous
// tidal-note: Q-0146. The call uses only the int parameter of each function, and the second function's constraint subsumes the first one's. The parameter lists differ in the defaulted parameter, so OVL-30 does not apply. EDG chooses the more constrained function.
template <class T> concept Known = sizeof(T) > 0;
template <class T> concept Small = Known<T> && sizeof(T) < 100;
template <class T> struct Pick {
  static int f(int) requires Known<T>;
  static int f(int, int = 0) requires Small<T>;
};
int x = Pick<int>::f(1);
