// tidal-rules: TSP-22
// tidal-expect: run
// tidal-note: A member of an explicitly specialized class, defined outside the class without template <>.
template <class T> struct Box { int get(); };
template <> struct Box<int> { int get(); };
int Box<int>::get() { return 1; }
int main() { return Box<int>{}.get() == 1 ? 0 : 1; }
