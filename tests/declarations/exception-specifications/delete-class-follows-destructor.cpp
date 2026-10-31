// tidal-rules: EXS-4
// tidal-expect: accept
struct Plain { ~Plain() {} };
struct Risky { ~Risky() noexcept(false) {} };
void g(Plain* p, Risky* r) { static_assert(noexcept(delete p)); static_assert(!noexcept(delete r)); }
