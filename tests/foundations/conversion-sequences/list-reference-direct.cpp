// tidal-rules: ICS-20
// tidal-expect: accept
// tidal-note: A list with one element of the referenced type, or of a derived class, binds the reference to the element. ICS-12 then applies, and ICS-30 compares two such bindings.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { int x; };
struct Derived : Base {};
int i = 0;
Derived d;
int pair[2];
One edit(int&);
Two edit(long);
static_assert(same<decltype(edit({i})), One>);
One look(int&);
Two look(const int&);
static_assert(same<decltype(look({i})), One>);
One take(int&&);
Two take(long);
static_assert(same<decltype(take({i})), Two>);
One keep(const int&&);
Two keep(const int&);
static_assert(same<decltype(keep({i})), Two>);
One part(Base&);
Two part(const Base&);
static_assert(same<decltype(part({d})), One>);
One whole(int (&)[2]);
Two whole(long);
static_assert(same<decltype(whole({pair})), One>);
