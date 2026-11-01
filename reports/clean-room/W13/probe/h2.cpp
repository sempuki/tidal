struct A { unsigned x : 4; unsigned : 0; unsigned y : 4; };
constexpr A make() { A a{}; a.x = 1; return a; }
static_assert(make().x == 1);
struct B { unsigned x : 4; unsigned : 3; unsigned y : 4; };
constexpr B mk() { B b{}; return b; }
static_assert(mk().y == 0);
int main() { return 0; }
