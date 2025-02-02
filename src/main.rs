use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Question {
    category: String,
    question: String,
    options: Vec<String>,
    answer: String,
}

async fn generate_question(category: &str) -> Result<Question, Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    // Use "llama3.2" as your model identifier.
    let model = "llama3.2".to_string();

    // Updated prompt to include detailed answer and MCQ options.
    let prompt = format!(
        "Generate a unique {} question with a detailed answer and multiple choice options in the following format:\n\
         Question: <question text>\n\
         Options: <option1>; <option2>; <option3>; <option4>\n\
         Answer: <short Answer for understanding>",
        category
    );

    // Call the Ollama model.
    let response = ollama.generate(GenerationRequest::new(model, prompt)).await;
    
    match response {
        Ok(resp) => {
            let generated_text = resp.response.trim();
            println!("Raw output: {}", generated_text);

            // Parse the output. Expecting three parts: Question, Options, and Answer.
            // We split using the keywords "Options:" and "Answer:".
            let question_split: Vec<&str> = generated_text.split("Options:").collect();
            if question_split.len() != 2 {
                return Err("Unexpected response format: missing 'Options:'".into());
            }
            let question_text = question_split[0].replace("Question:", "").trim().to_string();

            let options_split: Vec<&str> = question_split[1].split("Answer:").collect();
            if options_split.len() != 2 {
                return Err("Unexpected response format: missing 'Answer:'".into());
            }
            // Expect options separated by semicolons.
            let options: Vec<String> = options_split[0]
                .split(';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let answer_text = options_split[1].trim().to_string();

            Ok(Question {
                category: category.to_string(),
                question: question_text,
                options,
                answer: answer_text,
            })
        }
        Err(e) => {
            eprintln!("Detailed error from Ollama: {:?}", e);
            Err("Error in Ollama".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let category = "Quantitative Aptitude";
    match generate_question(category).await {
        Ok(question) => {
            println!("Generated Question:");
            println!("Category: {}", question.category);
            println!("Question: {}", question.question);
            println!("Options:");
            for (i, option) in question.options.iter().enumerate() {
                println!("  {}. {}", i + 1, option);
            }
            println!("Answer: {}", question.answer);
        },
        Err(e) => {
            eprintln!("Error generating question: {}", e);
        }
    }
    Ok(())
}
