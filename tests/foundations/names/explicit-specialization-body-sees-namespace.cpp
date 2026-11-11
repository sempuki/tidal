// tidal-rules: NAM-21
// tidal-expect: run
// tidal-note: An explicit specialization defined with a qualified name sees the members of the template's namespace in its body. Front-end ledger row G181: tidal-next's parser stops on the member type.
namespace shapes {
using length = long;
template <class T> struct sides;
}
struct Square {};
template <> struct shapes::sides<Square> {
  static constexpr length count = 4;
  length twice() const { return count * 2; }
};
int main() {
  static_assert(shapes::sides<Square>::count == 4);
  return shapes::sides<Square>{}.twice() == 8 ? 0 : 1;
}
