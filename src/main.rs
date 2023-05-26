mod api;
mod model;
mod repository;

use api::task::{complete_task, fail_task, get_task, pause_task, start_task, submit_task};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //ログの設定
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    //DynamDBへの接続設定
    let config = aws_config::load_from_env().await;
    //サーバーの設定
    HttpServer::new(move || {
        //DB接続のオブジェクトはスレッドセーフでなければならない
        let ddb_repo: DDBRepository = DDBRepository::init(String::from("task"), config.clone());
        let ddb_data = Data::new(ddb_repo);
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(submit_task)
            .service(fail_task)
            .service(pause_task)
            .service(start_task)
            .service(complete_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
