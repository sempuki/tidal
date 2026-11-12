namespace lib {
template <class E> constexpr E operator|(E a, E b) { return static_cast<E>(static_cast<int>(a) | static_cast<int>(b)); }
}
namespace app {
enum class P { r = 1, w = 2 };
using lib::operator|;
}
template <class E> concept has_or = requires(E a) { a | a; };
static_assert(has_or<app::P>);
int main() { return 0; }
