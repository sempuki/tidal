// tidal-rules: DED-13 TPL-28 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: Id<Tag> is the type int, and the aliased type does not use Tag. So the argument gives Tag no value.
template <class Tag> using Id = int;
template <class Tag> void tagged(Id<Tag>) {}
void use() { tagged(1); }
