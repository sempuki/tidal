// A nested namespace definition and an inline namespace for versions.
namespace lib::detail { constexpr int helper() { return 2; } }
namespace lib { inline namespace v2 { int version() { return 2; } } namespace v1 { int version() { return 1; } } }
int main() { return lib::version() + lib::v1::version() + lib::detail::helper() == 5 ? 0 : 1; }
