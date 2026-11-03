// tidal-rules: DEP-15 DEP-16
// tidal-expect: run
// tidal-note: Inside Box<T*>, the alias pointer names the type T*, so Box<pointer> has the partial specialization's argument list and names the current instantiation. The name Box<pointer>::type needs no typename.
template <class T> struct Box { using type = long; };
template <class T> struct Box<T*> {
  using pointer = T*;
  using type = int;
  int get() { Box<pointer>::type a = 2; return a; }
};
int main() { return Box<char*>{}.get() == 2 ? 0 : 1; }
