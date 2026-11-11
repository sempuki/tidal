// tidal-rules: DED-7 CV-6
// tidal-expect: run
// tidal-note: I5-3, I5 program p01-const-ref-function.cpp. For const T& and a function lvalue, the referred type const T is matched with int(char). const on a function type has no effect (CV-6), so T = int(char) makes the two types the same.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int twice(char c) { return c * 2; }
template <class T> int call(const T& f) { return same<T, int(char)> ? f('\1') : 0; }
int main() { return call(twice) == 2 ? 0 : 1; }
