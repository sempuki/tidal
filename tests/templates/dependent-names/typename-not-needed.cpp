// tidal-rules: DEP-21
// tidal-expect: run
struct Has { using type = int; };
typename Has::type outside = 1;
template <class T> int inside() {
  typename Has::type a = 2;
  typename T::type b = 3;
  return a + b;
}
template <class T> struct Box {
  using type = int;
  typename Box::type c = 4;
};
int main() { return outside + inside<Has>() + Box<int>{}.c == 10 ? 0 : 1; }
