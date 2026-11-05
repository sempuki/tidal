// G150: relational comparison of a pointer to a member of a base subobject
// and a pointer to a member of the derived class, in constant evaluation.
struct B { int b; };
struct D : B { int d; };
constexpr bool f() { D x{}; return &x.b < &x.d; }
static_assert(f());
int main() {}
