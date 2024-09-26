use core::fmt;

use pyo3::{exceptions::PyOSError, prelude::*};


#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule(name = "matrix_mul")]
mod matrix_mul {
    use super::*;


    #[pymodule_export]
    use super::sum_as_string;

    #[pyfunction]
    fn triple(x: usize) -> usize {
        x*3
    }

    #[pyfunction]
    fn version() -> usize {
        1
    }

    #[pymodule]
    mod functions {
        use pyo3::{exceptions::PyValueError, prelude::*};

        #[pyfunction]
        #[pyo3(name = "func_null")]
        fn return_null() -> usize {
            0
        }

        #[pyfunction]
        #[pyo3(signature = (num=-1))]
        fn return_eins(num: i32) -> i32 {
            num
        }

        /// return None if no value is provided
        #[pyfunction]
        #[pyo3(signature = (num=None), text_signature = "None")]
        fn return_either_or(num: Option<i32>) -> i32 {
            num.unwrap_or(0)
        }

        #[pyfunction]
        fn check_positive(x: i32) -> PyResult<()> {
            if x < 0 {
                Err(PyValueError::new_err("x is negative"))
            } else {
                Ok(())
            }
        }
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("version", m.getattr("version")?)
    }
}


#[derive(Debug)]
struct CustomError;

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "some error has occured")
    }
}

impl std::convert::From<CustomError> for PyErr {
    fn from(value: CustomError) -> Self {
        PyOSError::new_err(value.to_string())
    }
}

/// enums
/// enums.Enum
/// unit-only enums
#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
enum SampleEnum {
    Firsty,
    Secondy,
    Lastly = 10,
}

/// enums with structs and tuple variants
/// TupleEnum
#[pyclass]
enum Shape{
    Circle {radius: f32},
    Rectangle {width: f32, height: f32},
}


/// classes
/// 
#[pyclass]
struct ClassOne {
    id: i32
}

#[pymethods]
impl ClassOne {
    #[new]
    fn new(id: i32) -> Self {
        ClassOne { id }
    }
}

/// tuple class
/// 
/// TupleClas
#[pyclass]
struct TupleClas(String);


/// Inheritance
#[pyclass(subclass)]
struct BaseClass {
    id: i32
}


#[pymethods]
impl BaseClass {
    #[new]
    fn new(id: i32) -> Self {
        BaseClass { id }
    }

    pub fn methoda(&self) -> PyResult<i32> {
        Ok(self.id)
    }
}


#[pyclass(extends=BaseClass, subclass)]
struct SubClassA {
    #[pyo3(get, set)]
    id2: i32
}

#[pymethods]
impl SubClassA {
    #[new]
    fn new(id: i32) -> (Self, BaseClass) {
        (SubClassA{id2: id*34}, BaseClass::new(id))
    }

    pub fn methoda(self_: PyRef<'_, Self>) -> PyResult<i32> { //&self //&mut self
        let super_ = self_.as_super();
        super_.methoda()
    }

    #[getter]
    fn get_id(&self) -> PyResult<i32> {
        Ok(self.id2)
    }

    #[setter]
    pub fn set_id(&mut self, value: i32) -> PyResult<()> {
        self.id2 = value;
        Ok(())
    }

    // #[classmethod] #[staticmethod]
}