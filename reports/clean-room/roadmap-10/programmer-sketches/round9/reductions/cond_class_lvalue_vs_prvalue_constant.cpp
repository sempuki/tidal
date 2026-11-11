// The conditional operator in constant evaluation: a class lvalue against a
// prvalue of the same class gives a prvalue (COND-3). The run form works.
struct S { int n; };
constexpr int f(bool b) { S a{1}; S r = b ? a : S{2}; return r.n; }
static_assert(f(true) == 1);
static_assert(f(false) == 2);
int main() { return f(false) == 2 ? 0 : 1; }
