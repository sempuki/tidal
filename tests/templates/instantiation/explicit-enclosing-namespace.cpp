// tidal-rules: INST-21
// tidal-expect: accept
namespace lib {
template <class T> int get(T) { return 1; }
template <class T> struct Box {};
template int get(char);
}
template int lib::get(int);
template struct lib::Box<int>;
