// tidal-rules: CPT-8
// tidal-expect: accept
template <class T> concept Wide = sizeof(T) > 1;
template <template <class T> requires Wide<T> class Tmpl> struct Use {};
template <class T> requires Wide<T> struct Box {};
Use<Box> use;
