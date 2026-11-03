// tidal-rules: CLS-33
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The same as on x86-64 (empty-standard-layout-base-zero-size.cpp).
struct Empty {};
struct Shape : Empty { virtual int sides() const { return 0; } int id; };
struct PlainShape { virtual int sides() const { return 0; } int id; };
class Account : public Empty { public: int number; private: int balance = 0; };
class PlainAccount { public: int number; private: int balance = 0; };
static_assert(sizeof(Shape) == sizeof(PlainShape));
static_assert(sizeof(Account) == sizeof(PlainAccount));
