template <class T> constexpr int sz(const T& t) { if constexpr (requires { t.size(); }) return t.size(); else return 1; }
struct V { constexpr int size() const { return 4; } };
static_assert(sz(V{}) == 4 && sz(3) == 1);
int main() {}
