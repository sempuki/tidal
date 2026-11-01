// tidal-rules: TPL-47
// tidal-expect: accept
// tidal-note: Tag<int> and Tag<char> each have their own static member, so the two members have different addresses.
template <class T> struct Tag { static inline int count = 0; };
static_assert(&Tag<int>::count != &Tag<char>::count);
static_assert(&Tag<int>::count == &Tag<int>::count);
