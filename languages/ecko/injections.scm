; The body of a `sql { ... }` block is SQL. Its {holes} split it into several
; sql_text nodes, which `injection.combined` parses as one document. Zed
; highlights it only when a SQL language is installed (the SQL extension).
((sql_text) @injection.content
  (#set! injection.language "sql")
  (#set! injection.combined))
