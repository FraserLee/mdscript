use pyo3::{prelude::*, types::{PyString, PyTuple, PyModule}};

pub fn execute_md(md: &str) -> String {
    let code = //include_str!("../execute-dot-md/execute_md.py");
r#"
def test(iii):
    return "AAA" + iii + "CCC"
"#;

    let fn_name = "test";
    let fn_input = "BBB";
    run_python_code(code, fn_name, fn_input)
}

fn run_python_code(code: &str, fn_name: &str, fn_input: &str) -> String {

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::from_code(py, code, "", "").unwrap();

    let func = module.getattr(fn_name).unwrap();
    let args = PyTuple::new(py, &[fn_input]);

    func.call1(args).unwrap().cast_as::<PyString>().unwrap().to_string()
}





