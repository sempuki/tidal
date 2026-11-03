// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-note: A class shares its name with two overloaded functions and a function template.
struct status { int code; };
int status(int code);
int status(int code, int detail);
template <class T> int status(T* p);
