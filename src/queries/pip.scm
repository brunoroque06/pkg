((table
  (bare_key) @table
  (pair
    (bare_key) @key
    (array
      (string) @value)))
  (#eq? @table "project")
  (#eq? @key "dependencies"))

((table
  (bare_key) @table
  (pair
    (bare_key) @key
    (array
      (string) @value)))
  (#eq? @table "dependency-groups"))
