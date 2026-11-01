// tidal-rules: TSP-22 TSP-8
// tidal-expect: run
// tidal-note: A member function of an explicitly specialized class is an ordinary member, so it can have a default argument.
template <class T> struct Box { int get(int); };
template <> struct Box<char> { int get(int = 1); };
int Box<char>::get(int v) { return v; }
int main() { return Box<char>{}.get() == 1 ? 0 : 1; }
