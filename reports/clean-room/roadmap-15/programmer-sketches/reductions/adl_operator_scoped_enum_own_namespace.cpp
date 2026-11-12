namespace app {
enum class P { r = 1, w = 2 };
constexpr P operator|(P a, P b) { return static_cast<P>(static_cast<int>(a) | static_cast<int>(b)); }
}
static_assert(static_cast<int>(app::P::r | app::P::w) == 3);
int main() { return 0; }
