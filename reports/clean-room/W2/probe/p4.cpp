constexpr bool admit(bool s, bool b, bool e) { return s || b && e; }
constexpr bool ratio(int u, int q) { return q >= 0 && u / q < 2; }
static_assert(!ratio(1, 0));
int main() { return admit(false, false, false); }
