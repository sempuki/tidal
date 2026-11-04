// tidal-rules: CNS-40
// tidal-expect: accept
// tidal-note: The two templates name different concepts with the same meaning, so they are two templates (CNS-27). They have one name and one function type, and they accept the same argument lists. CNS-40 needs no diagnostic, and every compiler accepts the declarations. A call of fit would be ambiguous on every compiler.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> concept Large = sizeof(T) > 1;
template <class T> void fit(T) requires Wide<T>;
template <class T> void fit(T) requires Large<T>;
