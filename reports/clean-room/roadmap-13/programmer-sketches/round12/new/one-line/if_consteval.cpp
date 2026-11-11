// if consteval picks a path in a constant expression and another at run time.
constexpr int where() { if consteval { return 1; } else { return 2; } }
static_assert(where() == 1);
int main() { int r = where(); return r - 2; }
