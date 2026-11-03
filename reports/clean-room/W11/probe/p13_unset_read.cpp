struct T { int v[4]; };
constexpr T make() { T t; for (int i = 0; i < 3; ++i) t.v[i] = i + 1; return t; }
constexpr T table = make();
static_assert(table.v[3] == 0);
int main() { return 0; }
