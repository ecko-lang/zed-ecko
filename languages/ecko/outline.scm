(comment) @annotation

(function_definition
  "async"? @context
  "fn" @context
  name: (identifier) @name
  parameters: (parameters
    "(" @context
    ")" @context)) @item

(type_definition
  "type" @context
  name: (identifier) @name) @item

(template_definition
  "template" @context
  name: (identifier) @name) @item

(source_file
  (let_declaration
    ["let" "mut"] @context
    name: (identifier) @name) @item)

(source_file
  (assignment
    target: (identifier) @name) @item)

(export_statement
  (assignment
    target: (identifier) @name) @item)
