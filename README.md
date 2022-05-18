Small REPL for messing around with [egg](https://docs.rs/egg/0.6.0/egg/index.html).

- `e1 ~> e2` adds a new rewrite rule
- `e` runs equality saturation using all previously defined rewrite rules on `e`
- `e` (where `e` is a term that contains unification variables `?x`) queries the term stored in current egraph

Example in `lc`.
