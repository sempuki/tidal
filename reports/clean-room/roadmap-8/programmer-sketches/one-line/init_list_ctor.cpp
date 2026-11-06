namespace std { using size_t = decltype(sizeof 0); template <class E> class initializer_list { const E* b; size_t n; constexpr initializer_list(const E* p, size_t c) : b(p), n(c) {} public: constexpr initializer_list() : b(nullptr), n(0) {} constexpr const E* begin() const { return b; } constexpr const E* end() const { return b + n; } }; }
struct Sum { int s = 0; constexpr Sum(std::initializer_list<int> l) { for (int x : l) s += x; } };
static_assert(Sum{1, 2, 3}.s == 6);
int main() { Sum s{4, 5}; return s.s == 9 ? 0 : 1; }
