# `Option<T>`

| is_some() -> bool          |                             | is_none() -> bool               |                             |                              |              |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|------------------------------|--------------|
| Some(_)                    | true                        | Some(_)                         | false                       |                              |              |
| None                       | false                       | None                            | true                        |                              |              |
|                            |                             |                                 |                             |                              |              |
| ok_or(err) -> Result<T, E> |                             | ok_or_else(err) -> Result<T, E> |                             |                              |              |
| Some(t)                    | Ok(t)                       | Some(t)                         | Ok(t)                       |                              |              |
| None                       | Err(err)                    | None                            | Err(err())                  |                              |              |
|                            |                             |                                 |                             |                              |              |
| unwrap_or(def) -> T        |                             | unwrap_or_else(f) -> T          |                             | unwrap_or_default() -> T     |              |
| Some(t)                    | t                           | Some(t)                         | t                           | Some(t)                      | t            |
| None                       | def                         | None                            | f()                         | None                         | T::default() |
|                            |                             |                                 |                             |                              |              |
| unwrap() -> T              |                             | expect(msg) -> T                |                             |                              |              |
| Some(t)                    | t                           | Some(t)                         | t                           |                              |              |
| None                       | panic!()                    | None                            | panic!(msg)                 |                              |              |
|                            |                             |                                 |                             |                              |              |
| iter() -> Iter<T>          |                             | iter_mut() -> IterMut<T>        |                             |                              |              |
| Some(t)                    | iter.next() == Some(&t)     | Some(t)                         | iter.next() == Some(&mut t) |                              |              |
| None                       | iter.next() == None         | None                            | iter.next() == None         |                              |              |
|                            |                             |                                 |                             |                              |              |
| map(f) -> Option<U>        |                             | map_or(default, f) -> U         |                             | map_or_else(default, f) -> U |              |
| Some(t)                    | Some(f(t))                  | Some(t)                         | f(t)                        | Some(t)                      | f(t)         |
| None                       | None                        | None                            | default                     | None                         | default()    |
|                            |                             |                                 |                             |                              |              |
| and(optb) -> Option<U>     |                             | and_then(f) -> Option<U>        |                             |                              |              |
| Some(_)                    | optb                        | Some(t)                         | f(t)                        |                              |              |
| None                       | None                        | None                            | None                        |                              |              |
|                            |                             |                                 |                             |                              |              |
| or(optb) -> Option<T>      |                             | or_else(f) -> Option<T>         |                             |                              |              |
| Some(_)                    | self                        | Some(_)                         | self                        |                              |              |
| None                       | optb                        | None                            | f()                         |                              |              |
|                            |                             |                                 |                             |                              |              |
| as_ref() -> Option<&T>     |                             | as_mut() -> Option<&mut T>      |                             |                              |              |
| Some(t)                    | Some(&t)                    | Some(t)                         | Some(&mut t)                |                              |              |
| None                       | None                        | None                            | None                        |                              |              |
|                            |                             |                                 |                             |                              |              |
| take() -> Option<T>        |                             | cloned() -> Option<T>           |                             |                              |              |
| Some(t)                    | Some(t) (self becomes None) | Some(&t)                        | Some(t) (clones t)          |                              |              |
| None                       | None                        | None                            | None                        |                              |              |

---

# `Result<T, E>`

| is_ok() -> bool          |                | is_err() -> bool             |            |                         |       |
|--------------------------|----------------|------------------------------|------------|-------------------------|-------|
| Ok(_)                    | true           | Ok(_)                        | false      |                         |       |
| Err(_)                   | false          | Err(_)                       | true       |                         |       |
|                          |                |                              |            |                         |       |
| ok() -> Option<T>        |                | err() -> Option<E>           |            |                         |       |
| Ok(t)                    | t              | Ok(_)                        | None       |                         |       |
| Err(_)                   | None           | Err(e)                       | e          |                         |       |
|                          |                |                              |            |                         |       |
| map(op) -> Result<U, E>  |                | map_err(op) -> Result<T, F>  |            |                         |       |
| Ok(t)                    | Ok(op(t))      | Ok(t)                        | Ok(t)      |                         |       |
| Err(e)                   | Err(e)         | Err(e)                       | Err(op(e)) |                         |       |
|                          |                |                              |            |                         |       |
| and(res) -> Result<U, E> |                | and_then(op) -> Result<U, E> |            |                         |       |
| Ok(_)                    | res            | Ok(t)                        | op(t)      |                         |       |
| Err(e)                   | Err(e)         | Err(e)                       | Err(e)     |                         |       |
|                          |                |                              |            |                         |       |
| or(res) -> Result<T, F>  |                | or_else(op) -> Result<T, F>  |            |                         |       |
| Ok(t)                    | Ok(t)          | Ok(t)                        | Ok(t)      |                         |       |
| Err(_)                   | res            | Err(e)                       | op(e)      |                         |       |
|                          |                |                              |            |                         |       |
| unwrap() -> T            |                | unwrap_or(optb) -> T         |            | unwrap_or_else(op) -> T |       |
| Ok(t)                    | t              | Ok(t)                        | t          | Ok(t)                   | t     |
| Err(e)                   | panic!(e)      | Err(_)                       | optb       | Err(e)                  | op(e) |
|                          |                |                              |            |                         |       |
| expect(msg) -> T         |                |                              |            |                         |       |
| Ok(t)                    | t              |                              |            |                         |       |
| Err(e)                   | panic!(msg, e) |                              |            |                         |       |
|                          |                |                              |            |                         |       |
| unwrap_err() -> E        |                |                              |            |                         |       |
| Ok(t)                    | panic!(t)      |                              |            |                         |       |
| Err(e)                   | e              |                              |            |                         |       |
