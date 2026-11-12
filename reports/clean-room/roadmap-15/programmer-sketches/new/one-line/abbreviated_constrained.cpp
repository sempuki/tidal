template <class T> concept small = sizeof(T) <= 2;
constexpr int f(small auto) { return 1; }
constexpr int f(auto) { return 2; }
static_assert(f(char{}) == 1 && f(0LL) == 2);
int main() { return f(short{}) - 1; }
