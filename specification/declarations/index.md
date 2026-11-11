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
   translation_unit: {`declaration`}
   declaration: `block_decl`
              : | `function_definition`
              : | `namespace_def`
              : | `linkage_spec`
              : | `template_decl`
              : | `explicit_specialization`
              : | `explicit_instantiation`
              : | `deduction_guide`
              : | `concept_definition`
              : | ";"
   block_decl: `simple_decl`
             : | `sb_decl`
             : | `alias_decl`
             : | `enum_opaque`
             : | `static_assert_decl`
             : | `using_decl`
             : | `using_directive`
             : | `using_enum_decl`
             : | `namespace_alias`
   simple_decl: {`attribute_spec` | `alignment_specifier`} `decl_specifiers`
              :    [`init_declarator` {"," `init_declarator`}] ";"
   init_declarator: `declarator` [`init`]
                  : | `declarator` `requires_clause`
   decl_specifiers: `decl_specifier` {`decl_specifier`} {`attribute_spec`}
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
                 : | `typename_specifier`
   simple_type: "signed" | "unsigned" | "short" | "long" | "int"
              : | "char" | "wchar_t" | "char8_t" | "char16_t" | "char32_t"
              : | "bool" | "float" | "double" | "void"
              : | [`nested_name`] `identifier`
              : | [`nested_name`] `template_id`
```

`translation_unit` is the start symbol of the grammar. It derives the
tokens of one translation unit after preprocessing
({doc}`/foundations/translation`). Some productions on other pages are
outside what the start symbol derives:

- A *shape* production gives the form of a declaration or an expression
  that a general production also derives, such as a copy constructor.
  The page names the form so that its rules can refer to it.
- A *preprocessing* production gives source lines that preprocessing
  reads before the start symbol applies, such as an `#include` line.
