struct T { int v[4]; };
constexpr T make() { T t; for (int i = 0; i < 3; ++i) t.v[i] = i + 1; return t; }
constexpr T table = make();
static_assert(table.v[2] == 3);
int main() { return table.v[3]; }
