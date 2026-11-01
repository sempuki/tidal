// tidal-rules: DEP-15
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: use the "typename" keyword
// tidal-note: Same<T> is the same type as Box<T>. Clang and GCC treat it as the current instantiation, EDG asks for typename (Q-0229).
template <class T> struct Box;
template <class U> using Same = Box<U>;
template <class T> struct Box {
  using type = int;
  int get() { Same<T>::type a = 4; return a; }
};
int main() { return Box<char>{}.get() == 4 ? 0 : 1; }
