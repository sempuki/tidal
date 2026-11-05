// tidal-rules: TYN-7
// tidal-expect: accept
// tidal-note: An explicit specialization and an explicit instantiation of a class template can name the class with a qualified name and a class key. They start with template, so they are not declarations of only a class key and a qualified name.
namespace lib { template <class T> struct Box { T v; }; }
template <> struct lib::Box<char>;
template struct lib::Box<int>;
extern template struct lib::Box<long>;
template <> struct lib::Box<char> { char c; };
static_assert(sizeof(lib::Box<char>) == 1);
