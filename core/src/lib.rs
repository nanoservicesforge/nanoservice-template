//! This is the core logic of your application. This is where you write code that solves your problems.
//! Connecting the core logic to web frameworks and sockets is done in the servers directory.
use nanoservices_utils::errors::{NanoServiceError, NanoServiceErrorStatus};

use kernel::{
    CalcWork,
    WorkType,
    Echo
};

/// Performs a calculation based on the work type and returns the result.
/// 
/// # Arguments
/// * `contract` - The contract to perform the calculation on.
/// 
/// # Returns
/// The contract with the result of the calculation.
pub async fn handle_contract_work(mut contract: CalcWork) -> Result<CalcWork, NanoServiceError> {
    let data1 = contract.input_data1;
    let data2 = contract.input_data2;
    if data1 < 0 || data2 < 0 {
        contract.error = Some(NanoServiceError::new(
            "Input data must be a positive integer".to_string(),
            NanoServiceErrorStatus::BadRequest)
        );
    } else {
        contract.result = match &contract.work_type.clone() {
            WorkType::Sum => Some(data1 + data2),
            WorkType::Diff   =>  Some(data1 - data2),
            WorkType::Mult  => Some(data1 * data2),
            WorkType::Div   => Some(data1 / data2)
        };
    }
    Ok(contract)
}


/// Echoes the message back to the client.
/// 
/// # Arguments
/// * `contract` - The contract to echo the message back.
/// 
/// # Returns
/// The contract with the echoed message.
pub async fn handle_contract_echo(mut contract: Echo) -> Result<Echo, NanoServiceError> {
    contract.name = format!("Hello: {}", contract.name);
    Ok(contract)
}
