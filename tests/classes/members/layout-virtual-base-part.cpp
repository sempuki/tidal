// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-note: x86-64. A base subobject of a class with a virtual base holds only the pointer, the bases that are not virtual, and the members. The virtual bases come once, at the end of the complete object.
struct Shared { int v; };
struct Number { int n; };
// Middle: pointer 0, y 8, Shared 12
struct Middle : virtual Shared { char y; };
// Middle part 0 to 9, z 9, Shared 12
struct Last : Middle { char z; };
static_assert(sizeof(Middle) == 16 && sizeof(Last) == 16);
// Middle part first for its pointer, Number 12, z 16, Shared 20
struct Second : Number, Middle { char z; };
static_assert(sizeof(Second) == 24);
// two virtual bases: pointer 0, c 8, First 12, Other 16
struct First { int a; };
struct Other { int b; };
struct Two : virtual First, virtual Other { char c; };
static_assert(sizeof(Two) == 24);
// Left part 0 to 9, Right part 16 to 25, g 25, Other 28, First 32
struct Left : virtual Other { char l; };
struct Right : virtual First { char r; };
struct Graph : Left, Right { char g; };
static_assert(sizeof(Graph) == 40);
