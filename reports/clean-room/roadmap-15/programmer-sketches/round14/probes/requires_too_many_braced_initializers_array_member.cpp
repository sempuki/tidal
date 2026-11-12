struct S { short x; int c[1]; };
template <class T> concept three = requires { T{1, 2, 3}; };
static_assert(!three<S>);
int main() {}
