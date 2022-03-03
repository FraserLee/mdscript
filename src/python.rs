use pyo3::{
    prelude::*,
    types::{PyList, PyModule, PyTuple},
};


// runs a few markdown processing steps written in python
// (the architecture of this application is halfway between "don't think about
// it" and "guess what would be cool", so maybe don't take this too seriously :)

pub fn execute_md(input: &str) -> String {
    let linker_md_output = python_to_list_s(
        include_str!("../linker-dot-md/linker.py"),
        "link",
        input,
    );
    let execute_md_output = python_to_list_v(
        include_str!("../execute-dot-md/execute_md.py"),
        "execute_md",
        &linker_md_output,
    );

    execute_md_output.join("")
}

// Runs fn_name in the given python code on the given input
// output should always be a list of strings
fn python_to_list_v(code: &str, fn_name: &str, fn_input: &Vec<String>) -> Vec<String> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let func = PyModule::from_code(py, code, "", "").unwrap().getattr(fn_name).unwrap();

    let input_list = PyList::new(py, fn_input);
    let args = PyTuple::new(py, &[input_list]);

    func.call1(args).unwrap().extract().unwrap()
}

fn python_to_list_s(code: &str, fn_name: &str, fn_input: &str) -> Vec<String> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let func = PyModule::from_code(py, code, "", "").unwrap().getattr(fn_name).unwrap();

    let args = PyTuple::new(py, &[fn_input]);

    func.call1(args).unwrap().extract().unwrap()
}
