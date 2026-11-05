// G147: partial ordering of a base member template brought in by a using
// declaration against a derived member template.
struct A { template <class T> int f(T*) { return 1; } };
struct B : A { using A::f; template <class T> int f(T) { return 2; } };
int main() { B b; int i = 0; return b.f(&i) == 1 ? 0 : 1; }
