// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32, where a pointer takes 4 bytes. The rules are the same as on x86-64 (layout-member-sizes.cpp).
struct S { int r; void f(); };
struct Ref { char c; int& r; };
struct Data { char c; int S::* p; };
struct Function { char c; void (S::* p)(); };
struct Virtual { virtual void f() {} };
static_assert(sizeof(Ref) == 8 && alignof(Ref) == 4);
static_assert(sizeof(int S::*) == 4 && alignof(int S::*) == 4 && sizeof(Data) == 8);
static_assert(sizeof(void (S::*)()) == 8 && alignof(void (S::*)()) == 4 && sizeof(Function) == 12);
static_assert(sizeof(Virtual) == sizeof(void*) && alignof(Virtual) == alignof(void*));
