// tidal-rules: DED-13 TPL-28
// tidal-expect: accept
// tidal-note: The first parameter gives Tag no value, because Id<Tag> is int. The second parameter, or an explicit argument, gives the value.
template <class Tag> using Id = int;
template <class Tag> int tagged(Id<Tag>, Tag*) { return 1; }
template <class Tag> int named(Id<Tag>) { return 2; }
struct Meters;
Meters* where = nullptr;
int use() { return tagged(1, where) + named<Meters>(1); }
