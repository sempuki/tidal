constexpr auto squares = [] { struct T { int v[8]; } t{}; for (int i = 0; i < 8; ++i) t.v[i] = i * i; return t; }();
static_assert(squares.v[7] == 49);
int main() { return squares.v[3] == 9 ? 0 : 1; }
