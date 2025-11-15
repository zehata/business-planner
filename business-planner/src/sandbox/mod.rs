use gaol::profile::{Operation, OperationSupport, OperationSupportLevel};
use gaol::profile::{Profile};
use gaol::sandbox::{ChildSandbox, ChildSandboxMethods};

// Create the sandbox profile.
fn profile() -> Profile {
    // Set up the list of desired operations.
    let mut operations: Vec<Operation> = vec![];

    // Remove operations not supported by this OS. (Otherwise the creation of the profile will
    // fail.)
    operations.retain(|operation| {
        println!("{:?}: {:?}", operation, operation.support());
        matches!(operation.support(), OperationSupportLevel::NeverAllowed | OperationSupportLevel::CanBeAllowed)
    });

    Profile::new(operations).unwrap()
}

pub fn create_sandbox() {
    ChildSandbox::new(profile()).activate().unwrap();
}