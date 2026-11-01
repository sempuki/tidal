// tidal-rules: CPT-32
// tidal-expect: accept
// tidal-note: Never has no definition, and Broken's static assertion fails if Broken<int> is instantiated.
template <class T> struct Never;
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
template <class T> concept Names = requires { typename Never<T>; typename Broken<T>; };
static_assert(Names<int>);
