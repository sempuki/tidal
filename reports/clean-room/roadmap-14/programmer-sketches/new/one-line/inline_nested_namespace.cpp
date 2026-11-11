namespace lib::inline v2 { constexpr int version() { return 2; } }
namespace lib::v1 { constexpr int version() { return 1; } }
static_assert(lib::version() == 2 && lib::v1::version() == 1 && lib::v2::version() == 2);
int main() { return lib::version() - 2; }
