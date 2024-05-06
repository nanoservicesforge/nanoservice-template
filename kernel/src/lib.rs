//! This kernel acts as a compilation bridge between the client and server code housing the contracts that can be
//! sent to the server via the client.
use futures::{sink::SinkExt, StreamExt};
use nanoservices_utils::{
    create_contract_handler,
    errors::{NanoServiceError, NanoServiceErrorStatus},
    networking::codec::BincodeCodec
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;


/// Type of operation that the math server will perform.
/// 
/// # Fields
/// * `Sum` - Adds two numbers together.
/// * `Diff` - Subtracts two numbers.
/// * `Div` - Divides two numbers.
/// * `Mult` - Multiplies two numbers.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum WorkType {
    Sum,
    Diff,
    Div,
    Mult
}


/// A contract that houses the sending and response data.
/// 
/// # Fields
/// * `input_data1` - The first number to perform the operation on.
/// * `input_data2` - The second number to perform the operation on.
/// * `work_type` - The type of operation to perform.
/// * `result` - The result of the operation.
/// * `error` - The error if the operation failed.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CalcWork {
    pub input_data1: i32,
    pub input_data2: i32,
    pub work_type: WorkType,
    pub result: Option<i32>,
    pub error: Option<NanoServiceError>,
}


/// A basic contract for echoing a message.
/// 
/// # Fields
/// * `name` - The name to echo back to.
/// * `error` - The error if the operation failed.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Echo {
    pub name: String,
    pub error: Option<NanoServiceError>,
}


create_contract_handler!(
    TestContractHandler, // this handler struct is created by the macro
    CalcWork,
    Echo
);
