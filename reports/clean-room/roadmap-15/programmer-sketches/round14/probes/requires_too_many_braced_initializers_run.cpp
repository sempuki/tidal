struct C { int r; };
struct S { short x; C c; };
template <class T> concept init3 = requires { T{1, 2}; } && !requires { T{1, 2, 3}; };
int main() { return init3<S> ? 0 : 1; }
