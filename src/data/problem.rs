use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::get::get_data;

/**
 * Represents a problem information from Solved.ac API.
 */
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProblemInfo {
    /**
     * The unique identifier of the problem.
     */
    pub problemId: u32,
    /**
     * The title of the problem in Korean.
     */
    pub titleKo: String,
    /**
     * Whether the problem is solvable or not.
     */
    pub isSolvable: bool,
    /**
     * Whether the problem is partial or not.
     */
    pub isPartial: bool,
    /**
     * The level of the problem.
     */
    pub level: u32,
    /**
     * Whether the problem is a sprout or not.
     */
    pub sprout: bool,
    /**
     * Whether the problem gives no rating or not.
     */
    pub givesNoRating: bool,
    /**
     * The average number of tries to solve the problem.
     */
    pub averageTries: f64,
    /**
     * Whether the problem is official or not.
     */
    pub official: bool,
}

/**
 * Implementation of ProblemInfo.
 */
impl ProblemInfo {
    /**
     * Creates a new instance of ProblemInfo from a problem ID.
     *
     * # Example
     * ```
     * let problem_id = "1234".to_string();
     * let problem_info = ProblemInfo::new(problem_id).await?;
     * println!("Problem title: {}", problem_info.titleKo);
     * ```
     *
     * # Errors
     * Returns a `reqwest::Error` if the API request fails.
     */
    #[allow(dead_code)]
    pub async fn new(problem_id: String) -> Result<Self, reqwest::Error> {
        let url: String = "https://solved.ac/api/v3/problem/show".to_string();

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("x-solvedac-language".to_string(), "ko".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());
        
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".to_string();
        headers.insert("User-Agent".to_string(), user_agent);
    
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert("problemId".to_string(), problem_id);
    
        match get_data(url, Some(headers), Some(query)).await {
            Ok(data) => {
                let json_data = data.json::<ProblemInfo>().await?;

                Ok(json_data)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_vaild() {
        // Test with a valid problem ID
        let problem_id = "1000".to_string();
        let result = ProblemInfo::new(problem_id).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new_invaild() {
        // Test with an invalid problem ID
        let problem_id = "invalid".to_string();
        let result = ProblemInfo::new(problem_id).await;
        assert!(result.is_err());
    }
}