# Declarations

A declaration introduces a name and gives its properties. These pages
give the rules for each part of a declaration.

:::{tidal-pages}
auto
type-names
storage-linkage
inline
declarators
alignment
volatile
initialization/index
structured-bindings
enumerations
namespaces
linkage-specifications
attributes
functions/index
exception-specifications
constexpr
static-assert
:::

:::{toctree}
:hidden:
:maxdepth: 2

auto
type-names
storage-linkage
inline
declarators
alignment
volatile
initialization/index
structured-bindings
enumerations
namespaces
linkage-specifications
attributes
functions/index
exception-specifications
constexpr
static-assert
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   declaration: `block_decl`
              : | `function_definition`
              : | `namespace_def`
              : | `linkage_spec`
              : | `template_decl`
              : | `explicit_specialization`
              : | ";"
   block_decl: `simple_decl`
             : | `sb_decl`
             : | `alias_decl`
             : | `enum_opaque`
             : | `static_assert_decl`
             : | `using_decl`
             : | `using_directive`
             : | `namespace_alias`
   simple_decl: {`attribute_spec` | `alignment_specifier`} `decl_specifiers`
              :    [`init_declarator` {"," `init_declarator`}] ";"
   init_declarator: `declarator` [`init`]
   decl_specifiers: `decl_specifier` {`decl_specifier`}
   decl_specifier: `type_specifier`
                 : | `storage_class`
                 : | "typedef" | "friend" | "inline" | "virtual" | `explicit_spec`
                 : | "constexpr" | "consteval" | "constinit"
   type_specifiers: `type_specifier` {`type_specifier`}
   type_specifier: `simple_type`
                 : | `cv`
                 : | `elaborated_type`
                 : | `class_def`
                 : | `union_def`
                 : | `enum_def`
                 : | `placeholder`
                 : | `decltype_spec`
   simple_type: "signed" | "unsigned" | "short" | "long" | "int"
              : | "char" | "wchar_t" | "char8_t" | "char16_t" | "char32_t"
              : | "bool" | "float" | "double" | "void"
              : | [`nested_name`] `identifier`
              : | [`nested_name`] `template_id`
```
