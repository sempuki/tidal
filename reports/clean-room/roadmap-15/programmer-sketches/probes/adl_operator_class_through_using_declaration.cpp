namespace app { struct P { int v; }; }
namespace lib { constexpr app::P operator|(app::P a, app::P b) { return {a.v | b.v}; } }
namespace app { using lib::operator|; }
static_assert((app::P{1} | app::P{2}).v == 3);
int main() { return 0; }
