// tidal-rules: REF-2 ICS-11
// tidal-expect: accept
// tidal-note: Gap review RV7-A-5. Source is not related to Item, and its conversion function gives an Item xvalue. A const lvalue reference and an rvalue reference both bind directly to the object that the conversion function returns, so no temporary is made. Every compiler accepts the static_assert (writer probe roadmap-11/writer-probes/rv7-points.json).
struct Item { int value; };
Item shared{1};
struct Source { constexpr operator Item&&() const { return static_cast<Item&&>(shared); } };
constexpr bool const_binds_result() { const Item& ref = Source{}; return &ref == &shared; }
constexpr bool rvalue_binds_result() { Item&& ref = Source{}; return &ref == &shared; }
static_assert(const_binds_result());
static_assert(rvalue_binds_result());
