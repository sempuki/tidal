namespace app { enum class P { r = 1, w = 2 }; }
namespace lib {
constexpr app::P operator|(app::P a, app::P b) { return static_cast<app::P>(static_cast<int>(a) | static_cast<int>(b)); }
}
namespace app { using lib::operator|; }
static_assert(static_cast<int>(app::P::r | app::P::w) == 3);
int main() { return 0; }
