// tidal-rules: TSP-3 TSP-26
// tidal-expect: run
// tidal-note: Outer<char> and Inner<int> are both fixed, with one template <> for each.
template <class T> struct Outer {
  template <class U> struct Inner { int get() { return 0; } };
};
template <> template <> int Outer<char>::Inner<int>::get() { return 1; }
int main() { return Outer<char>::Inner<int>{}.get() == 1 && Outer<int>::Inner<int>{}.get() == 0 ? 0 : 1; }
