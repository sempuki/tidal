// tidal-rules: NS-4
// tidal-expect: accept
// tidal-note: An unnamed namespace inside a named namespace. The named namespace uses a function and a class of the unnamed namespace without qualification.
namespace outer {
namespace {
constexpr int helper() { return 3; }
struct Tag { int v = 4; };
}
constexpr int total = helper() + Tag{}.v;
}
static_assert(outer::total == 7);
static_assert(outer::helper() == 3);
