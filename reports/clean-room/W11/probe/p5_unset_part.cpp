struct T { int v[4]; };
constexpr T make() { T t; for (int i = 0; i < 3; ++i) t.v[i] = i; return t; }
constexpr T table = make();
int main() { return table.v[0]; }
