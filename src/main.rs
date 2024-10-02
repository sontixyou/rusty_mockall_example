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

fn main() {
    println!("Hello, world!");
}
