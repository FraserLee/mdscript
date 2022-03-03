use pyo3::{
    prelude::*,
    types::{PyList, PyModule, PyTuple},
};

// runs a few markdown processing steps written in python
// (the architecture of this application is halfway between "don't think about
// it" and "guess what would be cool", so maybe don't take this too seriously :)

pub fn execute_md(input: &str) -> String {
    let input_vec = input
        .split("\n")
        .into_iter()
        .map(|s| s.to_string() + "\n")
        .collect::<Vec<_>>();
    let execute_md_output = run_python_code(
        include_str!("../execute-dot-md/execute_md.py"),
        "execute_md",
        input_vec,
    );

    execute_md_output.join("")
}

// Runs the function in the given code, assuming it takes a list of strings
// '\n' terminated as input and returns the same as output.
fn run_python_code(code: &str, fn_name: &str, fn_input: Vec<String>) -> Vec<String> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::from_code(py, code, "", "").unwrap();

    let func = module.getattr(fn_name).unwrap();

    let input_list = PyList::new(py, fn_input);
    let args = PyTuple::new(py, &[input_list]);

    func.call1(args).unwrap().extract().unwrap()
}
