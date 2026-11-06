// notes 5: T::type with T an incomplete class.
struct X;
template <class T> int f(T*, typename T::type* = nullptr) { return 1; }
int f(...) { return 2; }
int main() { X* p = nullptr; return f(p) == 2 ? 0 : 1; }
