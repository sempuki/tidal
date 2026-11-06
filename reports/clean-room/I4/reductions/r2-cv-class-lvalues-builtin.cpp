struct A { operator int() const volatile { return 0; } };
const A x{};
volatile A y{};
int main() { return true ? x : y; }
