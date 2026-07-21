## `doxygen-bindgen` Crate

The `doxygen-bindgen` crate transforms Doxygen-style comments into Rustdoc-style markdown documentation. It helps convert Doxygen annotations used in C/C++ code into the appropriate Rustdoc format, making it easier to migrate or document codebases in Rust.

### Usage

```toml
[build-dependencies]
doxygen-bindgen = "0.1"
```

```rust
use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct ProcessComments;

impl ParseCallbacks for ProcessComments {
    fn process_comment(&self, comment: &str) -> Option<String> {
        match doxygen_bindgen::transform(comment) {
            Ok(res) => Some(res),
            Err(err) => {
                println!("cargo:warning=Problem processing doxygen comment: {comment}\n{err}");
                None
            }
        }
    }
}

bindgen::builder()
   .parse_callbacks(Box::new(ProcessComments))
```

### Example

```
/**
 * Creates a new registry key or opens an existing one, and it associates the key with a transaction.
 * 
 * @param[out] KeyHandle A pointer to a handle that receives the key handle.
 * @param[in] DesiredAccess The access mask that specifies the desired access rights.
 * @param[in] ObjectAttributes A pointer to an OBJECT_ATTRIBUTES structure that specifies the object attributes.
 * @param[in] TitleIndex Reserved.
 * @param[in, optional] Class A pointer to a UNICODE_STRING structure that specifies the class of the key.
 * @param[in] CreateOptions The options to use when creating the key.
 * @param[in] TransactionHandle A handle to the transaction.
 * @param[out, optional] Disposition A pointer to a variable that receives the disposition value.
 * @return NTSTATUS Successful or errant status.
 */
 ```

 Doxygen markup for [NtCreateKeyTransacted](https://docs.rs/phnt/latest/phnt/ffi/fn.NtCreateKeyTransacted.html) is transformed into:

```markdown
Creates a new registry key or opens an existing one, and it associates the key with a transaction.
# Arguments

* `KeyHandle` [out]  - A pointer to a handle that receives the key handle.
* `DesiredAccess` [in]  - The access mask that specifies the desired access rights.
* `ObjectAttributes` [in]  - A pointer to an OBJECT_ATTRIBUTES structure that specifies the object attributes.
* `TitleIndex` [in]  - Reserved.
* `Class` [in, optional]  - A pointer to a UNICODE_STRING structure that specifies the class of the key.
* `CreateOptions` [in]  - The options to use when creating the key.
* `TransactionHandle` [in]  - A handle to the transaction.
* `Disposition` [out, optional]  - A pointer to a variable that receives the disposition value.
# Returns

NTSTATUS Successful or errant status.
```


### Available Doxygen commands

| **Doxygen**                   | **Markdown**                  |
|-------------------------------|-------------------------------|
| `brief`, `short`              |                               |
| `param`                       | ``# Arguments\n\n* `name` -`` |
| `see`, `sa`                   | ``# See also\n\n> [`ref`]``   |
| `ref`                         | ``[`ref`]``                   |
| `a`, `e`, `em`                | _word_                        |
| `b`                           | **word**                      |
| `c`, `p`                      | `word`                        |
| `note`                        | `> **Note** `                 |
| `since`                       | `> **Since** `                |
| `deprecated`                  | `> **Deprecated** `           |
| `remark`, `remarks`           | `> `                          |
| `li`                          | `- `                          |
| `par`                         | `# `                          |
| `returns`, `return`, `result` | ``# Returns\n\n``             |
| `{`, `}`                      | Not implemented               |

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Contribution

We welcome contributions! If you'd like to help improve this crate, feel free to submit a pull request or open an issue.
