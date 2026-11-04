// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-note: x86-64. Each base class ends with 3 bytes of padding after its last member. Next adds one char. A size equal to the size of the base means that the base lends its end padding, and a larger size means that the base keeps it.
struct HasConstructor { HasConstructor() {} char x; };
struct HasMoveAssignment { HasMoveAssignment& operator=(HasMoveAssignment&&) = default; char x; };
struct PlainSmall { char x; };
struct Empty {};
template <class B> struct Next : B { char c; };
template <class B> constexpr bool lends = sizeof(Next<B>) == sizeof(B);
template <class B> constexpr bool keeps = sizeof(Next<B>) > sizeof(B);
// a base, a virtual function, a reference member
struct WithBase : Empty { int a; char b; };
struct WithVirtual { virtual void f() {} int a; char b; };
struct WithReference { int& r; int a; char b; };
static_assert(lends<WithBase> && lends<WithVirtual> && lends<WithReference>);
// user-provided constructors of every kind
struct Converting { Converting(int) {} int a; char b; };
struct Explicit { explicit Explicit(int) {} int a; char b; };
struct Template { template <class T> Template(T) {} int a; char b; };
struct Copying { Copying() = default; Copying(const Copying&) {} int a; char b; };
struct Moving { Moving() = default; Moving(Moving&&) {} int a; char b; };
struct OutOfLine { OutOfLine(); int a; char b; };
OutOfLine::OutOfLine() = default;
static_assert(lends<Converting> && lends<Explicit> && lends<Template>);
static_assert(lends<Copying> && lends<Moving> && lends<OutOfLine>);
// copy assignment by value, and members whose class lends
struct ByValue { int a; char b; ByValue& operator=(ByValue) { return *this; } };
struct MemberLends { HasConstructor m; int a; char b; };
struct ArrayLends { HasConstructor m[1]; int a; char b; };
static_assert(lends<ByValue> && lends<MemberLends> && lends<ArrayLends>);
// these keep their end padding
struct MemberKeeps { PlainSmall m; int a; char b; };
struct MemberMoveDefaulted { HasMoveAssignment m; int a; char b; };
struct ConstMember { const int a; char b; };
struct PointerMember { int* p; int a; char b; };
struct PrivateFunction { int a; char b; private: int get(); };
struct PrivateStatic { int a; char b; private: static int count; };
struct DeletedDestructor { ~DeletedDestructor() = delete; int a; char b; };
struct DefaultedAssignment { DefaultedAssignment& operator=(const DefaultedAssignment&) = default; int a; char b; };
static_assert(keeps<MemberKeeps> && keeps<MemberMoveDefaulted> && keeps<ConstMember> && keeps<PointerMember>);
static_assert(keeps<PrivateFunction> && keeps<PrivateStatic> && keeps<DeletedDestructor> && keeps<DefaultedAssignment>);
// a derived class whose base lends: its own member and the next one share that padding
struct Built { Built() {} int a; char b; };
struct Middle : Built { char d; };
static_assert(sizeof(Middle) == 8 && sizeof(Next<Middle>) == 8);
// an ordinary member never lends its padding
struct Ordinary { Built part; char c; };
static_assert(sizeof(Ordinary) == 12);
