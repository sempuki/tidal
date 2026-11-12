namespace lib {
template <class E> constexpr E operator|(E a, E b) { return static_cast<E>(static_cast<int>(a) | static_cast<int>(b)); }
}
namespace app {
enum class P { r = 1, w = 2 };
using lib::operator|;
constexpr int g() { return static_cast<int>(P::r | P::w); }
}
static_assert(app::g() == 3);
int main() { return 0; }
