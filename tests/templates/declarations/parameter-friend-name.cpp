// tidal-rules: TPL-18
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: template parameter "T" may not be redeclared in this scope
// tidal-note: A friend function declaration with the name of the parameter declares no name in the class template's scope. EDG rejects the declaration when Holder<int> is instantiated (Q-0198).
template <class T> struct Holder { friend void T(); };
Holder<int> h;
