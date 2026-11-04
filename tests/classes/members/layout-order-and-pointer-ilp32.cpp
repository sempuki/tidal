// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32, where the pointer for virtual calls takes 4 bytes and double has alignment 4 (TYP-2). The order is the same as on x86-64 (layout-order-and-pointer.cpp).
struct Number { int x; };
struct Letter { char y; };
struct Dynamic { virtual void f() {} };
struct Other { virtual void h() {} };
struct Shared { int v; };
// Letter 0, Number 4, z 8
struct Listed : Letter, Number { char z; };
static_assert(sizeof(Listed) == 12);
// pointer 0, Number 4, y 8
struct OwnPointer : Number { virtual void f() {} int y; };
static_assert(sizeof(OwnPointer) == 12);
// Dynamic 0, Number 4, y 8
struct UsesBase : Number, Dynamic { virtual void g() {} int y; };
static_assert(sizeof(UsesBase) == 12);
// Dynamic 0, Other 4, y 8
struct Both : Dynamic, Other { int y; };
static_assert(sizeof(Both) == 12);
// pointer 0, y 4, z 5 in the padding, Shared 8
struct Middle : virtual Shared { char y; };
struct Last : Middle { char z; };
static_assert(sizeof(Middle) == 12 && sizeof(Last) == 12);
// a 0, d 4, e 12, size 16
struct Padded { char a; double d; char e; };
static_assert(sizeof(Padded) == 16 && alignof(Padded) == 4);
