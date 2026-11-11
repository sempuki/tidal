struct C { int r; };
struct S { short x; C c; };
template <class T> concept three = requires { T{1, 2, 3}; };
static_assert(!three<S>);
int main() {}
