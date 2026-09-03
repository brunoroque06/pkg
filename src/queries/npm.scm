((document
  (object
    (pair
      key: (string
        (string_content) @group)
      value: (object
        (pair
          key: (string
            (string_content) @key)
          value: (string
            (string_content) @value))))))
  (#any-of? @group "dependencies" "devDependencies"))
