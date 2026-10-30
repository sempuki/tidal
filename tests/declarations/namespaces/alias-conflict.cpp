// tidal-rules: NS-5
// tidal-expect: reject
namespace A {}
namespace B {}
namespace L = A;
namespace L = B;
