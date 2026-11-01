// tidal-rules: INST-18
// tidal-expect: run
template <class T> int get(T) { return 1; }
template <> int get(int) { return 2; }
template int get(int);
template <class T> struct Box { int bad() { return T::nothing; } };
template <> struct Box<int> { int bad() { return 3; } };
template struct Box<int>;
int main() { return get(0) == 2 && Box<int>{}.bad() == 3 ? 0 : 1; }
