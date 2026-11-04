// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-note: x86-64 and -m32 alike. Each copy assignment operator form of CPY-3, when user-provided, makes the class lend its end padding. An operator= template and an operator= for another type do not. A constructor template also makes the class lend its padding.
template <class B> struct Next : B { char c; };
template <class B> constexpr bool lends = sizeof(Next<B>) == sizeof(B);
template <class B> constexpr bool keeps = sizeof(Next<B>) > sizeof(B);
struct ByReference { int a; char b; ByReference& operator=(ByReference&) { return *this; } };
struct ByConst { int a; char b; ByConst& operator=(const ByConst&) { return *this; } };
struct ByVolatile { int a; char b; ByVolatile& operator=(volatile ByVolatile&) { return *this; } };
struct ByConstVolatile { int a; char b; ByConstVolatile& operator=(const volatile ByConstVolatile&) { return *this; } };
struct ByValue { int a; char b; ByValue& operator=(ByValue) { return *this; } };
struct ByConstValue { int a; char b; ByConstValue& operator=(const ByConstValue) { return *this; } };
static_assert(lends<ByReference> && lends<ByConst> && lends<ByVolatile>);
static_assert(lends<ByConstVolatile> && lends<ByValue> && lends<ByConstValue>);
struct AssignTemplate { int a; char b; template <class T> AssignTemplate& operator=(const T&) { return *this; } };
struct AssignOther { int a; char b; AssignOther& operator=(int) { return *this; } };
static_assert(keeps<AssignTemplate> && keeps<AssignOther>);
struct PackConstructor { template <class... T> PackConstructor(T...) {} int a; char b; };
static_assert(lends<PackConstructor>);
