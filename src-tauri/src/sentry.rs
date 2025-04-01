use tauri_plugin_sentry::{minidump, sentry};

const ENVIRONMENT: &str = dotenv!("ENVIRONMENT");
const SENTRY_DSN: &str = dotenv!("SENTRY_DSN");

pub(crate) fn setup() -> sentry::ClientInitGuard {
    let client = sentry::init((
        SENTRY_DSN,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(std::borrow::Cow::Borrowed(ENVIRONMENT)),
            debug: ENVIRONMENT != "production",
            ..Default::default()
        },
    ));

    let _guard = minidump::init(&client);

    client
}
