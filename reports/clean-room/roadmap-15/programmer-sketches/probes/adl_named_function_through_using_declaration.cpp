namespace lib {
template <class E> constexpr int f(E) { return 1; }
}
namespace app {
enum class P { r = 1, w = 2 };
using lib::f;
}
static_assert(f(app::P::r) == 1);
int main() { return 0; }
