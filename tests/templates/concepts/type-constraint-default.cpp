// tidal-rules: CPT-17
// tidal-expect: accept
// tidal-note: The default char fails Wide, but no template-id uses the default.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T = char> struct Box {};
Box<int> box;
