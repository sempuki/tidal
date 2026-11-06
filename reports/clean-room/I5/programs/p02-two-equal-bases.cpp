// notes 7: two Base<int> subobjects give T = int twice.
template <class T> struct Base {};
struct L : Base<int> {};
struct R : Base<int> {};
struct Both : L, R {};
template <class T> int up(Base<T>&) { return 1; }
int up(...) { return 2; }
int main() { Both b; return up(b) == 2 ? 0 : 1; }
