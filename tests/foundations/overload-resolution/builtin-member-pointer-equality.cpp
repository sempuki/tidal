// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: Member pointers and std::nullptr_t have == and != candidates.
struct Item { int v; };
struct ToMember { operator int Item::*() const; };
struct ToNull { operator decltype(nullptr)() const; };
bool a = ToMember{} == nullptr;
bool b = ToMember{} != ToMember{};
bool c = ToNull{} == nullptr;
