// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-error-clang: use 'template' keyword to treat 'box' as a dependent template name
// tidal-error-gcc: non-template .box. used as template
// tidal-error-edg: nontype "T::box" is not a template
struct Has { template <class U> struct box {}; };
template <class T> struct Use { using alias = T::box<int>; };
Use<Has>::alias a;
