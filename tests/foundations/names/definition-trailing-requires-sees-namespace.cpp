// tidal-rules: NAM-21
// tidal-expect: accept
// tidal-note: Gap review RV6-A-6. A trailing requires clause comes after the qualified name, so the definition outside N finds the concept Small in N without qualification. Every compiler accepts it.
namespace N {
template <class T> concept Small = sizeof(T) <= 8;
template <class T> int size(T) requires Small<T>;
}
template <class T> int N::size(T) requires Small<T> { return int(sizeof(T)); }
int main() { return N::size(1) == int(sizeof(int)) ? 0 : 1; }
