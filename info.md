### Function options

```bash
The `#[pyfunction]` attribute is used to define a Python function from a Rust function.
Once defined, the function needs to be added to a module using the `wrap_pyfunction!` macro.
```

function options

`#[pyo3(name = "...")]` be used to modify properties of the generated Python function.
eg

```rust
#[pyfunction]
#[pyo3(name = "no_args")]
fn no_args_py() -> usize {
    42
}

#[pymodule]
fn module_with_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(no_args_py, m)?)
}
```

`#[pyo3(signature = (...))]` for signature formatting
eg

```rust
#[pyfunction]
#[pyo3(signature = (**kwds))]
fn num_kwds(kwds: Option<&Bound<'_, PyDict>>) -> usize {
    kwds.map_or(0, |dict| dict.len())
}
```

or

```rust
#[pymethods]
impl MyClass {
    #[new]
    #[pyo3(signature = (num=-1))]
    fn new(num: i32) -> Self {
        MyClass { num }
    }

    #[pyo3(signature = (num=10, *py_args, name="Hello", **py_kwargs))]
    fn method(
        &mut self,
        num: i32,
        py_args: &Bound<'_, PyTuple>,
        name: &str,
        py_kwargs: Option<&Bound<'_, PyDict>>,
    ) -> String {
        let num_before = self.num;
        self.num = num;
        format!(
            "num={} (was previously={}), py_args={:?}, name={}, py_kwargs={:?} ",
            num, num_before, py_args, name, py_kwargs,
        )
    }

    fn make_change(&mut self, num: i32) -> PyResult<String> {
        self.num = num;
        Ok(format!("num={}", self.num))
    }
}
```

and we can use it like this

```python
mc = mymodule.MyClass()
print(mc.method(44, False, "World", 666, x=44, y=55))
print(mc.method(num=-1, name="World"))
print(mc.make_change(44, False))
```

also, using signature, we can also do something like this

```rust
    /// Returns a copy of `x` increased by `amount`.
    ///
    /// If `amount` is unspecified or `None`, equivalent to `x + 1`.
    #[pyfunction]
    fn increment(x: u64, amount: Option<u64>) -> u64 {
        x + amount.unwrap_or(1)
    }
```

`#[pyo3(text_signature = "...")]` overwrites the function text signature

```rust
    /// This function adds two unsigned 64-bit integers.
    #[pyfunction]
    #[pyo3(signature = (a, b=0, /), text_signature = None)]
    fn add(a: u64, b: u64) -> u64 {
        a + b
    }
```

this will give `pyo3_test.add.__text_signature__ == None` when asserted

### Per-argument options

`#[pyo3(from_py_with = "...")]` to specify a custom function to convert the function argument from Python to the desired Rust type, instead of using the default FromPyObject. Eg

```rust
fn get_length(obj: &Bound<'_, PyAny>) -> PyResult<usize> {
    obj.len()
}


#[pyfunction]
fn object_length(#[pyo3(from_py_with = "get_length")] argument: usize) -> usize {
    argument
}
```

of course, `#[pyfn(m)]` is just syntactic sugar for `#[pyfunction]`

### Raising errors

to raise an exception from a #[pyfunction], change the return type T to PyResult<T>. When the function returns an Err it will raise a Python exception

```rust
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn check_positive(x: i32) -> PyResult<()> {
    if x < 0 {
        Err(PyValueError::new_err("x is negative"))
    } else {
        Ok(())
    }
}
```

As a more complete example, the following snippet defines a Rust error named CustomIOError. It then defines a From<CustomIOError> for PyErr, which returns a PyErr representing Python's OSError. Therefore, it can use this error in the result of a #[pyfunction] directly, relying on the conversion if it has to be propagated into a Python exception.

```rust
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::fmt;

#[derive(Debug)]
struct CustomIOError;

impl std::error::Error for CustomIOError {}

impl fmt::Display for CustomIOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Oh no!")
    }
}

impl std::convert::From<CustomIOError> for PyErr {
    fn from(err: CustomIOError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}

pub struct Connection {/* ... */}

fn bind(addr: String) -> Result<Connection, CustomIOError> {
    if &addr == "0.0.0.0" {
        Err(CustomIOError)
    } else {
        Ok(Connection{ /* ... */})
    }
}

#[pyfunction]
fn connect(s: String) -> Result<(), CustomIOError> {
    bind(s)?;
    // etc.
    Ok(())
}

fn main() {
    Python::with_gil(|py| {
        let fun = pyo3::wrap_pyfunction_bound!(connect, py).unwrap();
        let err = fun.call1(("0.0.0.0",)).unwrap_err();
        assert!(err.is_instance_of::<PyOSError>(py));
    });
}
```

## Python classes

Defining a new class

```rust
use pyo3::prelude::*;

#[pyclass]
struct MyClass {
    inner: i32,
}

// A "tuple" struct
#[pyclass]
struct Number(i32);

// PyO3 supports unit-only enums (which contain only unit variants)
// These simple enums behave similarly to Python's enumerations (enum.Enum)
#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
enum MyEnum {
    Variant,
    OtherVariant = 30, // PyO3 supports custom discriminants.
}

// PyO3 supports custom discriminants in unit-only enums
#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
enum HttpResponse {
    Ok = 200,
    NotFound = 404,
    Teapot = 418,
    // ...
}

// PyO3 also supports enums with Struct and Tuple variants
// These complex enums have sligtly different behavior from the simple enums above
// They are meant to work with instance checks and match statement patterns
// The variants can be mixed and matched
// Struct variants have named fields while tuple enums generate generic names for fields in order _0, _1, _2, ...
// Apart from this both types are functionally identical
#[pyclass]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    RegularPolygon(u32, f64),
    Nothing(),
}
```

#### restrictions

To integrate Rust types with Python, PyO3 needs to place some restrictions on the types which can be annotated with #[pyclass]. In particular, they must have no lifetime parameters, no generic parameters, and must implement Send.

### Constructor

By default, it is not possible to create an instance of a custom class from Python code. To declare a constructor, you need to define a method and annotate it with the #[new] attribute. Only Python's **new** method can be specified, **init** is not available.

```rust
#[pymethods]
impl Number {
    #[new]
    fn new(value: i32) -> Self {
        Number(value)
    }
}
```

or we can also handle error

```rust
#[pymethods]
impl Nonzero {
    #[new]
    fn py_new(value: i32) -> PyResult<Self> {
        if value == 0 {
            Err(PyValueError::new_err("cannot be zero"))
        } else {
            Ok(Nonzero(value))
        }
    }
}
```

we then add the class to a module like so

```rust
#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Number>()?;
    Ok(())
}
```

### inheritance

To get a parent class from a child, use PyRef instead of &self for methods, or PyRefMut instead of &mut self.

```rust

#[pyclass(subclass)]
struct BaseClass {
    val1: usize,
}

#[pymethods]
impl BaseClass {
    #[new]
    fn new() -> Self {
        BaseClass { val1: 10 }
    }

    pub fn method1(&self) -> PyResult<usize> {
        Ok(self.val1)
    }
}

#[pyclass(extends=BaseClass, subclass)]
struct SubClass {
    val2: usize,
}

#[pymethods]
impl SubClass {
    #[new]
    fn new() -> (Self, BaseClass) {
        (SubClass { val2: 15 }, BaseClass::new())
    }

    fn method2(self_: PyRef<'_, Self>) -> PyResult<usize> {
        let super_ = self_.as_super(); // Get &PyRef<BaseClass>
        super_.method1().map(|x| x * self_.val2)
    }
}

#[pyclass(extends=SubClass)]
struct SubSubClass {
    val3: usize,
}

#[pymethods]
impl SubSubClass {
    #[new]
    fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(SubClass::new()).add_subclass(SubSubClass { val3: 20 })
    }

    fn method3(self_: PyRef<'_, Self>) -> PyResult<usize> {
        let base = self_.as_super().as_super(); // Get &PyRef<'_, BaseClass>
        base.method1().map(|x| x * self_.val3)
    }

    fn method4(self_: PyRef<'_, Self>) -> PyResult<usize> {
        let v = self_.val3;
        let super_ = self_.into_super(); // Get PyRef<'_, SubClass>
        SubClass::method2(super_).map(|x| x * v)
    }

      fn get_values(self_: PyRef<'_, Self>) -> (usize, usize, usize) {
          let val1 = self_.as_super().as_super().val1;
          let val2 = self_.as_super().val2;
          (val1, val2, self_.val3)
      }

    fn double_values(mut self_: PyRefMut<'_, Self>) {
        self_.as_super().as_super().val1 *= 2;
        self_.as_super().val2 *= 2;
        self_.val3 *= 2;
    }

    #[staticmethod]
    fn factory_method(py: Python<'_>, val: usize) -> PyResult<PyObject> {
        let base = PyClassInitializer::from(BaseClass::new());
        let sub = base.add_subclass(SubClass { val2: val });
        if val % 2 == 0 {
            Ok(Py::new(py, sub)?.to_object(py))
        } else {
            let sub_sub = sub.add_subclass(SubSubClass { val3: val });
            Ok(Py::new(py, sub_sub)?.to_object(py))
        }
    }
}

```

### Object properties

For simple cases where a member variable is just read and written with no side effects, you can declare getters and setters

```rust
#[pyclass]
struct MyClass {
    #[pyo3(get, set)]
    num: i32,
}
```

The above would make the num field available for reading and writing as a self.num Python property
We can override names as such `#[pyo3(get, set, name = "custom_name")]`

We can also explicity add setters and getters like so

```rust
#[pymethods]
impl MyClass {
    #[getter]
    fn get_num(&self) -> PyResult<i32> {
        Ok(self.num)
    }

    #[setter]
    fn set_num(&mut self, value: i32) -> PyResult<()> {
        self.num = value;
        Ok(())
    }
}
```

To define class methods similar to @classmethod

```rust
#[pymethods]
impl MyClass {
    #[classmethod]
    fn cls_method(cls: &Bound<'_, PyType>) -> PyResult<i32> {
        Ok(10)
    }
}
```

constructors that accept arguments like so

```rust
#[pymethods]
impl BaseClass {
    #[new]
    #[classmethod]
    fn py_new(cls: &Bound<'_, PyType>) -> PyResult<Self> {
        // Get an abstract attribute (presumably) declared on a subclass of this class.
        let subclass_attr: Bound<'_, PyAny> = cls.getattr("a_class_attr")?;
        Ok(Self(subclass_attr.unbind()))
    }
}
```

and static methods

```rust
#[pymethods]
impl MyClass {
    #[staticmethod]
    fn static_method(param1: i32, param2: &str) -> PyResult<i32> {
        Ok(10)
    }
}
```
