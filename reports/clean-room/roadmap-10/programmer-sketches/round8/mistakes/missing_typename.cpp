template <class T> int f() { T::type x = 0; return x; }
struct S { using type = int; };
int main() { return f<S>(); }
