namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
struct S { int v; };
constexpr int f() { union U { S s; char c; } u{.c = 0}; ::new (static_cast<void*>(&u.s)) S{3}; return u.s.v; }
static_assert(f() == 3);
int main() {}
