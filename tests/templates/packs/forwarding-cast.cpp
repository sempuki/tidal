// tidal-rules: PACK-23
// tidal-expect: run
// tidal-note: static_cast<T&&>(x) gives an lvalue for the lvalue arguments and an xvalue for the others, so each call chooses the matching overload.
int calls = 0;
void sink(int&) { calls += 1; }
void sink(int&&) { calls += 10; }
template <class... T> void relay(T&&... x) { (sink(static_cast<T&&>(x)), ...); }
int main() {
  int a = 0;
  relay(a, 1, a, static_cast<int&&>(a));
  return calls == 22 ? 0 : 1;
}
