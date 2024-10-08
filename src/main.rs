//
// mockitoを使ったHTTPリクエストのモック
//

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[cfg(not(test))]
static PATH: &str = "https://jsonplaceholder.typicode.com/todos/";
#[cfg(test)]
static PATH: &str = "http://0.0.0.0:1234/todos/";

#[tokio::main]
async fn main() {
    let todo_id = 1;
    // NOTE:
    // 変数urlの値を変更したときは、テストがコケるべきときにコケてくれない。
    // これを守るためにRustではどう書けば良いのか?
    // 毎回モックサーバーを起動するのが手間だけど、Rustでは基本的な方法？
    let url = "https://jsonplaceholder.typicode.com/todos/";
    match run(url, todo_id).await {
        Ok(result) => println!("{:?}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn run(url: &str, todo_id: u32) -> Result<Todo, String> {
    if !validate_domain(url) {
        return Err("Invalid domain".to_string());
    }

    let url = format!("{}{}", url, todo_id);

    match fetch_todo_api(&url).await {
        Ok(todo) => Ok(todo),
        Err(err) => Err(format!("Error: {}", err)),
    }
}

fn validate_domain(url: &str) -> bool {
    url == PATH
}

async fn fetch_todo_api(url: &str) -> Result<Todo, Error> {
    let response = reqwest::get(url).await?;
    response.json::<Todo>().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        let opts = mockito::ServerOpts {
            host: "0.0.0.0",
            port: 1234,
            ..Default::default()
        };
        let mut server = mockito::Server::new_with_opts(opts);
        let json_body =
            r#"{"userId": 1, "id": 1, "title": "delectus aut autem", "completed": false}"#;

        let mock = server
            .mock("GET", "/todos/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_body)
            .create_async()
            .await;

        let url = server.url() + "/todos/";
        let todo = run(&url, 1).await.unwrap();
        let expect_todo = Todo {
            user_id: 1,
            id: 1,
            title: "delectus aut autem".to_string(),
            completed: false,
        };
        assert_eq!(todo, expect_todo);

        mock.assert_async().await;
    }
}
