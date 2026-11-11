// tidal-rules: DEP-4
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: typename
// tidal-note: I6-6. sizeof(N) is not value-dependent, so Box<sizeof(N)> is a type that does not depend on N, and Box<sizeof(N)>::type needs no typename at block scope. EDG treats sizeof(N) as value-dependent and asks for typename (Q-0432).
template <decltype(sizeof 0) K> struct Box { using type = int; };
template <int N> int read() {
  Box<sizeof(N)>::type value = N;
  return value;
}
int main() { return read<3>() == 3 ? 0 : 1; }
