// tidal-rules: DEP-18
// tidal-expect: run
template <class T> struct Base { static constexpr int missing = 3; };
template <class T> struct Box : Base<T> {
  int get() { return Box::missing; }
};
int main() { return Box<int>{}.get() == 3 ? 0 : 1; }
