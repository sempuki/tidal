// tidal-rules: CNS-14
// tidal-expect: accept
// tidal-note: Box<T> in User and in size names a specialization with dependent arguments. User<char> is never instantiated, and size is never called.
template <class T> requires (sizeof(T) > 1) struct Box {};
template <class T> struct User { Box<T>* p; };
User<char>* user;
template <class T> int size() { return sizeof(Box<T>*); }
