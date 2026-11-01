struct B { unsigned x : 4; unsigned : 3; unsigned y : 4; };
constexpr B mk() { B b{}; return b; }
static_assert(mk().y == 0);
int main() { B b{}; B c = b; return static_cast<int>(c.y); }
