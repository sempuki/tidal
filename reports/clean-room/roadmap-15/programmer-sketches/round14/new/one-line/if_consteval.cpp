constexpr int where() { if consteval { return 1; } else { return 2; } }
static_assert(where() == 1);
int main() { int r = where(); return r == 2 ? 0 : 1; }
