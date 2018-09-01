use std::boxed::Box;

use command::CommandFnDef;
use commands_processor::CommandProcessor;
use jsprint::JSprint;

fn dummy_command(jsprint: &mut JSprint, line: &str) {}

#[test]
fn register_command() {
    let mut processor = CommandProcessor::new();
    processor.register_command("test", Box::new(dummy_command));

    let result = processor.get_command("test");
    let expected = Box::new(dummy_command);

    assert!(result.is_some());

    let result_ptr: *const CommandFnDef = result.unwrap().as_ref();
    let expected_ptr: *const CommandFnDef = expected.as_ref();

    assert_eq!(result_ptr, expected_ptr);
}

#[test]
fn unknwon_command() {
    let processor = CommandProcessor::new();

    let result = processor.get_command("test");

    assert!(result.is_none());
}
