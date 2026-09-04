((table
  (_) @table
  (pair
    (_) @key
    (string) @value))
  (#any-of? @table "dependencies" "dev-dependencies" "build-dependencies"))

((table
  (_) @table
  (pair
    (_) @key
    (inline_table
      (pair
        (_) @property
        (string) @value))))
  (#any-of? @table "dependencies" "dev-dependencies" "build-dependencies")
  (#eq? @property "version"))
