| name        |    | return        |   | name          |    | return    |   | name               |    | return      | 
|-------------|----|---------------|---|---------------|----|-----------|---|--------------------|----|-------------|
| **is_some** | -> | bool          |   | **is_none**   | -> | bool      |   |                    |    |             |
| Some(_)     | -> | true          |   | Some(_)       | -> | false     |   |                    |    |             |
| None        | -> | false         |   | None          | -> | true      |   |                    |    |             |
|             |    |               |   |               |    |           |   |                    |    |             |
| **unwrap**  | -> | T             |   | **unwrap_or** | -> | T         |   | **unwrap_or_else** | -> | T           |
| Some(t)     | -> | t             |   | Some(t)       | -> | t         |   | Some(t)            | -> | t           |
| None        | -> | panic!()      |   | None          | -> | *default* |   | None               | -> | *default()* |
| **expect**  | -> | T             |   |               |    |           |   |                    |    |             |
| Some(t)     | -> | t             |   |               |    |           |   |                    |    |             |
| None        | -> | panic!(*msg*) |   |               |    |           |   |                    |    |             |
|             |    |               |   |               |    |           |   |                    |    |             |
| **map**     | -> | Option<U>     |   | **map_or**    | -> | U         |   | **map_or_else**    | -> | U           |
| Some(t)     | -> | *f(*t*)*      |   | Some(t)       | -> | *f(*t*)*  |   | Some(t)            | -> | *f(*t*)*    |
| None        | -> | None          |   | None          | -> | *default* |   | None               | -> | *default()* |
|             |    |               |   |               |    |           |   |                    |    |             |
| **and**     | -> | Option<U>     |   | **and_then**  | -> | Option<U> |   |                    |    |             |
| Some(_)     | -> | *optb*        |   | Some(t)       | -> | *f(*t*)*  |   |                    |    |             |
| None        | -> | None          |   | None          | -> | None      |   |                    |    |             |
|             |    |               |   |               |    |           |   |                    |    |             |
| **or**      | -> | Option<T>     |   | **or_else**   | -> | Option<T> |   |                    |    |             |
| Some(_)     | -> | self          |   | Some(_)       | -> | self      |   |                    |    |             |
| None        | -> | *optb*        |   | None          | -> | *optb()*  |   |                    |    |             |
