// tidal-rules: ICS-45
// tidal-expect: accept
// tidal-note: Direct initialization of a reference uses an explicit conversion function only when the result reaches the referenced type by a qualification adjustment alone. A derived-to-base step or an int to long step leaves the function out, for each kind of result.
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
struct Base { Base(); };
struct Derived : Base { Derived(); };
struct ToDerived { explicit operator Derived(); };
struct ToDerivedLvalue { explicit operator Derived&(); };
struct ToDerivedRvalue { explicit operator Derived&&(); };
struct ToInt { explicit operator int(); };
struct ToIntLvalue { explicit operator int&(); };
struct ToIntRvalue { explicit operator int&&(); };
static_assert(!makes<Base&&, ToDerived>);
static_assert(!makes<const Base&, ToDerived>);
static_assert(!makes<Base&&, ToDerivedLvalue>);
static_assert(!makes<const Base&, ToDerivedLvalue>);
static_assert(!makes<Base&&, ToDerivedRvalue>);
static_assert(!makes<const Base&, ToDerivedRvalue>);
static_assert(!makes<long&&, ToInt>);
static_assert(!makes<const long&, ToInt>);
static_assert(!makes<long&&, ToIntLvalue>);
static_assert(!makes<const long&, ToIntLvalue>);
static_assert(!makes<long&&, ToIntRvalue>);
static_assert(!makes<const long&, ToIntRvalue>);
