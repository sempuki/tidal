struct C { int r; };
struct S { short x; C c; };
template <class T> concept two = requires { T{1, 2}; };
static_assert(two<S>);
int main() {}
