// I3-1: Derived* to Middle* is a proper subsequence of Derived* to const Middle*.
struct Base {}; struct Middle : Base {}; struct Derived : Middle {};
constexpr int f(Middle*) { return 1; }
constexpr int f(const Middle*) { return 2; }
constexpr int g(Base*) { return 1; }
constexpr int g(const Middle*) { return 2; }
Derived d;
static_assert(f(&d) == 1);
int main() { return g(&d); }
