namespace lib { using size = decltype(sizeof 0); template <class T> struct count; }
struct Range {};
template <> struct lib::count<Range> { static constexpr size value = 2; };
static_assert(lib::count<Range>::value == 2);
int main() {}
