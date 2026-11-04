// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32, where the pointer for virtual calls takes 4 bytes. The parts are the same as on x86-64 (layout-virtual-base-part.cpp).
struct Shared { int v; };
struct Number { int n; };
// Middle: pointer 0, y 4, Shared 8
struct Middle : virtual Shared { char y; };
// Middle part 0 to 5, z 5, Shared 8
struct Last : Middle { char z; };
static_assert(sizeof(Middle) == 12 && sizeof(Last) == 12);
// Middle part first for its pointer, Number 8, z 12, Shared 16
struct Second : Number, Middle { char z; };
static_assert(sizeof(Second) == 20);
// two virtual bases: pointer 0, c 4, First 8, Other 12
struct First { int a; };
struct Other { int b; };
struct Two : virtual First, virtual Other { char c; };
static_assert(sizeof(Two) == 16);
// Left part 0 to 5, Right part 8 to 13, g 13, Other 16, First 20
struct Left : virtual Other { char l; };
struct Right : virtual First { char r; };
struct Graph : Left, Right { char g; };
static_assert(sizeof(Graph) == 24);
