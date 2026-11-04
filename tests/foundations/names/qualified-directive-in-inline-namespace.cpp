// tidal-rules: NAM-13
// tidal-expect: accept
// tidal-note: The using directive sits in an inline namespace of n. A qualified name n::v follows the directive.
namespace lib { int v = 1; }
namespace n { inline namespace in { using namespace lib; } }
int r = n::v;
