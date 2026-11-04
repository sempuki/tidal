// tidal-rules: CLS-34 CLS-35
// tidal-expect: accept
// tidal-note: x86-64. Each base has 3 bytes of tail padding after b. A derived member c goes into that padding for the bases in the second and third lines of checks, and after it for the others.
struct Plain { int a; char b; };                         // keeps its tail padding
struct Initialized { int a = 0; char b; };              // default member initializer
struct Built { Built() {} int a; char b; };             // user-provided constructor
struct Assigned { int a; char b; Assigned& operator=(const Assigned&) { return *this; } };
struct Ended { int a; char b; ~Ended() {} };            // user-provided destructor
struct Hidden { int a; private: char b; };              // private member
struct Guarded { int a; protected: char b; };           // protected member
struct Derived : Plain {}; // members in a base
template <class B> struct Next : B { char c; };
static_assert(sizeof(Next<Plain>) == 12 && sizeof(Next<Derived>) == 12);
static_assert(sizeof(Next<Initialized>) == 8 && sizeof(Next<Built>) == 8 && sizeof(Next<Assigned>) == 8);
static_assert(sizeof(Next<Ended>) == 8 && sizeof(Next<Hidden>) == 8 && sizeof(Next<Guarded>) == 8);
