union Cell { int i; float f; };
constexpr int test() { Cell c{.i = 3}; int a = c.i; c.f = 2.5f; float b = c.f; c.i = 4; return a + c.i + static_cast<int>(b * 2); }
static_assert(test() == 12);
int main() { return test() - 12; }
