// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-note: x86-64. A reference member and a pointer to data member take the size and alignment of a pointer. A pointer to member function takes two pointers, with the alignment of one. The pointer for virtual calls takes one pointer.
struct S { int r; void f(); };
struct Ref { char c; int& r; };
struct Data { char c; int S::* p; };
struct Function { char c; void (S::* p)(); };
struct Virtual { virtual void f() {} };
static_assert(sizeof(Ref) == 16 && alignof(Ref) == 8);
static_assert(sizeof(int S::*) == 8 && alignof(int S::*) == 8 && sizeof(Data) == 16);
static_assert(sizeof(void (S::*)()) == 16 && alignof(void (S::*)()) == 8 && sizeof(Function) == 24);
static_assert(sizeof(Virtual) == sizeof(void*) && alignof(Virtual) == alignof(void*));
