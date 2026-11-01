// tidal-rules: TPL-6
// tidal-expect: accept
// tidal-note: Each permitted kind of type for a constant template parameter.
enum class Color { red, green };
struct Point { int x; int y; };
struct Holder { int member; };
int object = 0;
void function() {}
template <int N> struct Int {};
template <bool B> struct Bool {};
template <Color C> struct Enum {};
template <int* P> struct Ptr {};
template <void (*F)()> struct FuncPtr {};
template <int& R> struct Ref {};
template <int Holder::*M> struct MemPtr {};
template <decltype(nullptr) N> struct Null {};
template <double D> struct Float {};
template <Point P> struct Class {};
Int<3> a;
Bool<true> b;
Enum<Color::green> c;
Ptr<&object> d;
FuncPtr<&function> e;
Ref<object> f;
MemPtr<&Holder::member> g;
Null<nullptr> h;
Float<1.5> i;
Class<Point{1, 2}> j;
