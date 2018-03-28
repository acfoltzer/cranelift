//! The `print-cfg` sub-command.
//!
//! Read a series of Cretonne IR files and print their control flow graphs
//! in graphviz format.

use std::borrow::Cow;

use cretonne::ir::Function;
use cretonne::cfg_printer::CFGPrinter;
use cton_reader::TestCommand;
use subtest::{self, SubTest, Context, Result as STResult};

/// Object implementing the `test print-cfg` sub-test.
struct TestPrintCfg;

pub fn subtest(parsed: &TestCommand) -> STResult<Box<SubTest>> {
    assert_eq!(parsed.command, "print-cfg");
    if !parsed.options.is_empty() {
        Err(format!("No options allowed on {}", parsed))
    } else {
        Ok(Box::new(TestPrintCfg))
    }
}

impl SubTest for TestPrintCfg {
    fn name(&self) -> Cow<str> {
        Cow::from("print-cfg")
    }

    fn needs_verifier(&self) -> bool {
        false
    }

    fn run(&self, func: Cow<Function>, context: &Context) -> STResult<()> {
        subtest::run_filecheck(&CFGPrinter::new(&func).to_string(), context)
    }
}
