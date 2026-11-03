// tidal-rules: CLS-33
// tidal-expect: accept
// tidal-note: Empty is empty and standard-layout, so as a base it takes no bytes. The derived classes are not standard-layout, one for its virtual function and one for its members in two accesses. Each has the size of the same class without the base.
struct Empty {};
struct Shape : Empty { virtual int sides() const { return 0; } int id; };
struct PlainShape { virtual int sides() const { return 0; } int id; };
class Account : public Empty { public: int number; private: int balance = 0; };
class PlainAccount { public: int number; private: int balance = 0; };
static_assert(sizeof(Shape) == sizeof(PlainShape));
static_assert(sizeof(Account) == sizeof(PlainAccount));
