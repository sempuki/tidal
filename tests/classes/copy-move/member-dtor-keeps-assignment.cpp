// tidal-rules: CPY-7
// tidal-expect: accept
struct Pinned { ~Pinned() = delete; };
class Hidden { ~Hidden() = default; };
struct Holder { Pinned pinned; Hidden hidden; };
void copy(Holder& to, const Holder& from) { to = from; }
void move(Holder& to, Holder& from) { to = static_cast<Holder&&>(from); }
