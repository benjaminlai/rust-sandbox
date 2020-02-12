## Naming Conventions

| Item                    | Convention                                            |
| ----------------------- | ----------------------------------------------------- |
| Crates                  | snake_case (but prefer single word)                   |
| Modules                 | snake_case                                            |
| Types                   | CamelCase                                             |
| Traits                  | CamelCase                                             |
| Enum variants           | CamelCase                                             |
| Functions               | snake_case                                            |
| Methods                 | snake_case                                            |
| General constructors    | new or with_more_details                              |
| Conversion constructors | from_some_other_type                                  |
| Local variables         | snake_case                                            |
| Static variables        | SCREAMING_SNAKE_CASE                                  |
| Constant variables      | SCREAMING_SNAKE_CASE                                  |
| Type parameters         | concise CamelCase, usually single uppercase letter: T |
| Lifetimes               | short, lowercase: 'a                                  |

In CamelCase, acronyms count as one word: use Uuid rather than UUID. In snake_case, acronyms are lower-cased: is_xid_start.

In snake_case or SCREAMING_SNAKE_CASE, a "word" should never consist of a single letter unless it is the last "word". So, we have btree_map rather than b_tree_map, but PI_2 rather than PI2.
