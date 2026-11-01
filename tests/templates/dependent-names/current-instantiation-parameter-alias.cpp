// tidal-rules: DEP-15 DEP-16
// tidal-expect: run
template <class T> struct Box {
  using self = T;
  using type = int;
  int get() { Box<self>::type local = 2; return local; }
};
int main() { return Box<char>{}.get() == 2 ? 0 : 1; }
