use nanoservices_utils::{
    register_contract_routes,
    errors::{NanoServiceError, NanoServiceErrorStatus},
};
use kernel::TestContractHandler;
use core::{
    handle_contract_work,
    handle_contract_echo
};


register_contract_routes!(
    TestContractHandler,                  // Struct handling contract serialization
    handle_contract_routes,               // Generate an overall contract handler function of this name
    CalcWork => handle_contract_work,   // Map a contract struct to existing handler function
    Echo => handle_contract_echo
);
