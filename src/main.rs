//
// mockallを使ったモックのサンプル
//
use mockall::predicate::*;
use mockall::*;

// モックしたい構造体やトレイトの定義
#[automock]
trait Database {
    fn query(&self, sql: &str) -> Vec<String>;
}

#[test]
fn test_database_query_calls() {
    // モックオブジェクトの作成
    let mut mock = MockDatabase::new();

    mock.expect_query()
        .with(eq("SELECT * FROM users"))
        .times(3) // この関数が3回呼ばれることを期待
        .returning(|_| vec!["user1".to_string(), "user2".to_string()]);

    for _ in 0..3 {
        mock.query("SELECT * FROM users");
    }

    // これは任意。これがなくても３回未満で呼ばれた場合テストが落ちる
    mock.checkpoint();
}

//
// mockitoを使ったHTTPリクエストのモック
//

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

async fn fetch_todo_api(url: &str) -> Result<Todo, Error> {
    let response = reqwest::get(url).await?;
    response.json::<Todo>().await
}

fn todo_details(todo: &Todo) -> String {
    format!(
        "Todo:\nUserId: {}\nId: {}\nTitle: {}\nCompleted: {}",
        todo.user_id, todo.id, todo.title, todo.completed
    )
}

#[tokio::main]
async fn main() {
    if let Some(todo_id) = std::env::args().nth(1) {
        // NOTE: 異なるドメインのAPIを叩く際には、モックサーバーをドメインごとに立てる必要がある。
        // https://hoge.com/todos/1も叩きたい場合を考えている
        let base_url = "https://jsonplaceholder.typicode.com/todos/";
        let url = format!("{}{}", base_url, todo_id);
        match fetch_todo_api(&url).await {
            Ok(todo) => println!("{}", todo_details(&todo)),
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        eprintln!("Error: Todo ID not provided");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_todo_api() {
        // NOTE:
        // こういうモック方法ではなく、RSpecかGolangのgockのように、モックを設定できないだろうか
        // 理想なクレート.new("http://jsonplaceholder.typicode.com")
        // .get("/todos/1")
        // .status(200)
        // .body(r#"{"userId": 1, "id": 1, "title": "delectus aut autem", "completed": false}"#)
        //
        // 理想なクレート.new("http://hoge.com")
        // .get("/todos/1")
        // .status(200)
        // .body(r#"{"userId": 100, "id": 100, "title": "Rust mock", "completed": false}"#)
        //
        // 利点
        // テストコード内のアサーションでは、引数のurlによって、どのモックを使うかを切り替える必要がなくなる。
        // 本番環境とテスト環境で同じコードを実行できるようになる。
        let mut server = mockito::Server::new_async().await;
        let path = "/todos/1";
        let json_body =
            r#"{"userId": 1, "id": 1, "title": "delectus aut autem", "completed": false}"#;

        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_body)
            .create_async()
            .await;

        let url = server.url() + path;
        let todo: Todo = fetch_todo_api(&url).await.unwrap();

        assert_eq!(todo.user_id, 1);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "delectus aut autem");
        assert!(!todo.completed);

        mock.assert_async().await;
    }
}
