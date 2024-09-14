use data::problem::ProblemInfo;

mod api;
mod data;

#[tokio::main]
async fn main() {
    // Test with a valid problem ID
    let problem_id: String = "1000".to_string();
    let result: Result<ProblemInfo, reqwest::Error> = ProblemInfo::new(problem_id).await;
    
    let problem_info: ProblemInfo = result.unwrap();
    eprint!("{:?}", problem_info);
}
