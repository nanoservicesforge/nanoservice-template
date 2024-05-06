use kernel::{
    TestContractHandler,
    CalcWork,
    Echo
};
use nanoservices_utils::errors::NanoServiceError;


#[tokio::main]
async fn main() -> Result<(), NanoServiceError> {

    let server = "0.0.0.0:8001";
    let calc_contract = TestContractHandler::CalcWork(CalcWork {
        input_data1: 10,
        input_data2: 10,
        work_type: kernel::WorkType::Mult,
        result: None,
        error: None,
    });
    let result = calc_contract.send_over_tcp(server).await?.CalcWork()?;
    println!("{:?}", result);

    let calc_contract2 = TestContractHandler::CalcWork(CalcWork {
        input_data1: 10,
        input_data2: 10,
        work_type: kernel::WorkType::Sum,
        result: None,
        error: None,
    });
    let result = calc_contract2.send_over_tcp(server).await?.CalcWork()?;
    println!("{:?}", result);

    let echo_contract = TestContractHandler::Echo(Echo {
        name: "World!".to_string(),
        error: None,
    });

    let result = echo_contract.send_over_tcp(server).await?.Echo()?;
    println!("{:?}", result);

    Ok(())
}
