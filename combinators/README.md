# `Option<T>`

| is_some() -> bool          |                             | is_none() -> bool               |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(_)                    | true                        | Some(_)                         | false                       |
| None                       | false                       | None                            | true                        |

## Proceed with the inner value

| unwrap() -> T              |                             | expect(msg) -> T                |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(t)                    | t                           | Some(t)                         | t                           |
| None                       | panic!()                    | None                            | panic!(msg)                 |

| unwrap_or(def) -> T        |                             | unwrap_or_else(f) -> T          |                             | unwrap_or_default() -> T |              |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|--------------------------|--------------|
| Some(t)                    | t                           | Some(t)                         | t                           | Some(t)                  | t            |
| None                       | def                         | None                            | f()                         | None                     | T::default() |

| map_or(default, f) -> U    |                             | map_or_else(default, f) -> U    |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(t)                    | f(t)                        | Some(t)                         | f(t)                        |
| None                       | default                     | None                            | default()                   |

## Option to Option Combinators

| map(f) -> Option<U>        |                             |
|----------------------------|-----------------------------|
| Some(t)                    | Some(f(t))                  |
| None                       | None                        |

| and(optb) -> Option<U>     |                             | and_then(f) -> Option<U>        |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(_)                    | optb                        | Some(t)                         | f(t)                        |
| None                       | None                        | None                            | None                        |

| or(optb) -> Option<T>      |                             | or_else(f) -> Option<T>         |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(_)                    | self                        | Some(_)                         | self                        |
| None                       | optb                        | None                            | f()                         |

## Proceed with a reference to the inner value

| as_ref() -> Option<&T>     |                             | as_mut() -> Option<&mut T>      |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(t)                    | Some(&t)                    | Some(t)                         | Some(&mut t)                |
| None                       | None                        | None                            | None                        |

## Take or clone the inner value

| take() -> Option<T>        |                             | cloned() -> Option<T>           |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(t)                    | Some(t) (self becomes None) | Some(&t)                        | Some(t) (clones t)          |
| None                       | None                        | None                            | None                        |

## Proceed with an iterator to the value (useful for iterator chaining)

| iter() -> Iter<T>          |                             | iter_mut() -> IterMut<T>        |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(t)                    | iter.next() == Some(&t)     | Some(t)                         | iter.next() == Some(&mut t) |
| None                       | iter.next() == None         | None                            | iter.next() == None         |

## Proceed with an Result, rather than an Option

| ok_or(err) -> Result<T, E> |                             | ok_or_else(err) -> Result<T, E> |                             |
|----------------------------|-----------------------------|---------------------------------|-----------------------------|
| Some(t)                    | Ok(t)                       | Some(t)                         | Ok(t)                       |
| None                       | Err(err)                    | None                            | Err(err())                  |

---

# `Result<T, E>`

|                            |                         |                                    |                             |                              |              |
|----------------------------|-------------------------|------------------------------------|-----------------------------|------------------------------|--------------|
| is_ok() -> bool            |                         | is_err() -> bool                   |                             |                              |              |
| Ok(_)                      | true                    | Ok(_)                              | false                       |                              |              |
| Err(_)                     | false                   | Err(_)                             | true                        |                              |              |
|                            |                         |                                    |                             |                              |              |
| ok() -> Option<T>          |                         | err() -> Option<E>                 |                             |                              |              |
| Ok(t)                      | t                       | Ok(_)                              | None                        |                              |              |
| Err(_)                     | None                    | Err(e)                             | e                           |                              |              |
|                            |                         |                                    |                             |                              |              |
| map(op) -> Result<U, E>    |                         | map_err(op) -> Result<T, F>        |                             |                              |              |
| Ok(t)                      | Ok(op(t))               | Ok(t)                              | Ok(t)                       |                              |              |
| Err(e)                     | Err(e)                  | Err(e)                             | Err(op(e))                  |                              |              |
|                            |                         |                                    |                             |                              |              |
| and(res) -> Result<U, E>   |                         | and_then(op) -> Result<U, E>       |                             |                              |              |
| Ok(_)                      | res                     | Ok(t)                              | op(t)                       |                              |              |
| Err(e)                     | Err(e)                  | Err(e)                             | Err(e)                      |                              |              |
|                            |                         |                                    |                             |                              |              |
| or(res) -> Result<T, F>    |                         | or_else(op) -> Result<T, F>        |                             |                              |              |
| Ok(t)                      | Ok(t)                   | Ok(t)                              | Ok(t)                       |                              |              |
| Err(_)                     | res                     | Err(e)                             | op(e)                       |                              |              |
|                            |                         |                                    |                             |                              |              |
| unwrap() -> T              |                         | expect(msg) -> T                   |                             |                              |              |
| Ok(t)                      | t                       | Ok(t)                              | t                           |                              |              |
| Err(e)                     | panic!(e)               | Err(e)                             | panic!(msg, e)              |                              |              |
|                            |                         |                                    |                             |                              |              |
| unwrap_or(optb) -> T       |                         | unwrap_or_else(op) -> T            |                             | unwrap_or_default(self) -> T |              |
| Ok(t)                      | t                       | Ok(t)                              | t                           | Ok(t)                        | t            |
| Err(_)                     | optb                    | Err(e)                             | op(e)                       | Err(e)                       | T::default() |
|                            |                         |                                    |                             |                              |              |
| unwrap_err() -> E          |                         |                                    |                             |                              |              |
| Ok(t)                      | panic!(t)               |                                    |                             |                              |              |
| Err(e)                     | e                       |                                    |                             |                              |              |
|                            |                         |                                    |                             |                              |              |
| as_ref() -> Result<&T, &E> |                         | as_mut() -> Result<&mut T, &mut E> |                             |                              |              |
| Ok(t)                      | &t                      | Ok(t)                              | &mut t                      |                              |              |
| Err(e)                     | &e                      | Err(e)                             | &mut e                      |                              |              |
|                            |                         |                                    |                             |                              |              |
| iter() -> Iter<T>          |                         | iter_mut() -> IterMut<T>           |                             |                              |              |
| Ok(t)                      | iter.next() == Some(&t) | Ok(t)                              | iter.next() == Some(&mut t) |                              |              |
| Err(e)                     | iter.next() == None     | Err(e)                             | iter.next() == None         |                              |              |

# Thanks

Tables generated with http://www.tablesgenerator.com/markdown_tables.
