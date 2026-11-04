// tidal-rules: OVL-30 OVL-36
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-clang-18: accept
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int\). is ambiguous
// tidal-note: Q-0146. As tie-constraints-default-parameter.cpp, with an ellipsis in place of the defaulted parameter. EDG and Clang 18 choose the more constrained function.
template <class T> concept Known = sizeof(T) > 0;
template <class T> concept Small = Known<T> && sizeof(T) < 100;
template <class T> struct Pick {
  static int f(int) requires Known<T>;
  static int f(int, ...) requires Small<T>;
};
int x = Pick<int>::f(1);
