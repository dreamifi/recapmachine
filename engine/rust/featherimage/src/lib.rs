
use pyo3::prelude::*;

#[pymodule]
fn featherimage(py: Python, module: &PyModule) -> PyResult<()> {
	
    #[pyfn(module, "hello")]
    fn hello(_py: Python) -> PyResult<String> {
        Ok(String::from("hello python!"))
    }
    Ok(())
}