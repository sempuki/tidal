// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: exception declarator cannot be qualified
// tidal-error-gcc: expected unqualified-id
// tidal-error-edg: qualified name is not allowed
namespace n { extern int x; }
void g() { try { } catch (int n::x) { } }
