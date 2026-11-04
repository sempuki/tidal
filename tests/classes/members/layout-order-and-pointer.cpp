// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-note: x86-64, where the pointer for virtual calls takes 8 bytes. Each size follows from the order of CLS-35 and from no other order.
struct Number { int x; };
struct Letter { char y; };
struct Dynamic { virtual void f() {} };
struct Other { virtual void h() {} };
struct Shared { int v; };
// bases in the order of the base list, then the members: Letter 0, Number 4, z 8
struct Listed : Letter, Number { char z; };
static_assert(sizeof(Listed) == 12);
// its own pointer at 0, before a base without one: pointer 0, Number 8, y 12
struct OwnPointer : Number { virtual void f() {} int y; };
static_assert(sizeof(OwnPointer) == 16);
// the first base with a pointer goes to 0: Dynamic 0, Number 8, y 12
struct UsesBase : Number, Dynamic { virtual void g() {} int y; };
static_assert(sizeof(UsesBase) == 16);
// two bases with pointers keep both: Dynamic 0, Other 8, y 16
struct Both : Dynamic, Other { int y; };
static_assert(sizeof(Both) == 24);
// a virtual base comes last: pointer 0, y 8, z 9 in the padding, Shared 12
struct Middle : virtual Shared { char y; };
struct Last : Middle { char z; };
static_assert(sizeof(Middle) == 16 && sizeof(Last) == 16);
// padding before each member, and at the end
struct Padded { char a; double d; char e; };
static_assert(sizeof(Padded) == 24 && alignof(Padded) == 8);
