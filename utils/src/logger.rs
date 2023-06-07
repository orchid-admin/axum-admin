use time::{format_description, UtcOffset};
use tracing::Level;
use tracing_subscriber::{fmt::time::OffsetTime, prelude::*, EnvFilter};

pub fn init(env_filter: Option<String>) {
    //格式化日志日期时间输出，处理时区问题
    let format = "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]";
    let time_format = OffsetTime::new(
        UtcOffset::current_local_offset().unwrap(),
        format_description::parse(format).unwrap(),
    );
    //INFO-日志输出配置
    let info_log_file_appender = tracing_appender::rolling::daily("./logs/info", "app-info.log")
        .with_max_level(Level::INFO)
        .with_min_level(Level::INFO);
    //WARN-日志输出配置
    let warn_log_file_appender = tracing_appender::rolling::daily("./logs/warn", "app-warn.log")
        .with_max_level(Level::WARN)
        .with_min_level(Level::WARN);
    //ERROR-日志输出配置
    let error_log_file_appender = tracing_appender::rolling::daily("./logs/error", "app-error.log")
        .with_max_level(Level::ERROR)
        .with_min_level(Level::ERROR);
    let all_files_appender = info_log_file_appender
        .and(warn_log_file_appender)
        .and(error_log_file_appender);
    //文件输出配置
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(all_files_appender)
        .with_line_number(true)
        .with_timer(time_format.clone())
        .compact();
    //控制台输出配置
    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_line_number(true)
        .with_timer(time_format.clone());

    let logger = tracing_subscriber::registry()
        .with(file_layer)
        .with(formatting_layer);
    match env_filter {
        Some(env_filter) => {
            let env_filter_layer = EnvFilter::builder().parse_lossy(env_filter);
            logger.with(env_filter_layer).init();
        }
        None => {
            logger.init();
        }
    };
}
