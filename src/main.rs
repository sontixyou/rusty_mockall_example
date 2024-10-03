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
use reqwest::Response;
use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize)]
struct ApiResponseBody {
    timestamp: String,
    holiday_name: String,
}

pub async fn fetch_holidays_jp_in_year() -> Result<Response, reqwest::Error> {
    let url = "https://holidays-jp.github.io/api/v1/2021/datetime.json";
    let response: Response = reqwest::get(url).await?;
    Ok(response)
}

#[tokio::test]
async fn test_fetch_holidays_jp_in_year() {
    // doc: https://docs.rs/mockito/latest/mockito/struct.ServerOpts.html
    let opts = mockito::ServerOpts {
        // NOTE: IPアドレスを指定するのみ
        // host: "https://holidays-jp.github.io",
        host: "0.0.0.0",
        ..Default::default()
    };
    let mut server_with_host = mockito::Server::new_with_opts(opts);

    let response_body = ApiResponseBody {
        timestamp: "2021-01-01".to_string(),
        holiday_name: "元日".to_string(),
    };

    let _m = server_with_host
        .mock("GET", "/api/v1/2021/datetime.json")
        .with_status(200)
        .with_header(
            reqwest::header::CONTENT_TYPE.to_string(),
            "application/json",
        )
        .with_body(json!(response_body).to_string());

    // モックサーバーにリクエストを送信
    let response = fetch_holidays_jp_in_year().await.unwrap();

    println!("response: {:?}", response);
    // server host and port: "0.0.0.0:65192"
    println!(
        "server host and port: {:?}",
        server_with_host.host_with_port()
    );
    // server host: "http://0.0.0.0:65192"
    println!("server host: {:?}", server_with_host.url());
}

fn main() {
    println!("Hello, world!");
}
