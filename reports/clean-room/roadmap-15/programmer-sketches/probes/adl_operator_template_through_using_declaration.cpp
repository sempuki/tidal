namespace lib {
template <class E> constexpr E operator|(E a, E b) { return static_cast<E>(static_cast<int>(a) | static_cast<int>(b)); }
}
namespace app {
enum class P { r = 1, w = 2 };
using lib::operator|;
}
static_assert(static_cast<int>(app::P::r | app::P::w) == 3);
int main() { return 0; }
