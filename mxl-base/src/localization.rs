use i18n_embed::{
    DefaultLocalizer, DesktopLanguageRequester, LanguageLoader, Localizer,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use rust_embed::RustEmbed;
use std::sync::OnceLock;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

// A language loader must always needs to be initialized by an init function.
// Do not use an automatic initialization implementation such as "std::sync::LazyLock",
// this can lead to a deadlock!
pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub(crate) fn init() {
    LANGUAGE_LOADER.get_or_init(|| {
        let loader = fluent_language_loader!();
        loader
            .load_fallback_language(&Localizations)
            .expect("Error while loading fallback language");

        let localizer = DefaultLocalizer::new(&loader, &Localizations);
        let requested_languages = DesktopLanguageRequester::requested_languages();
        if let Err(error) = localizer.select(&requested_languages) {
            log::error!("Error while loading language: {error}");
        }
        loader
    });
}

pub(crate) fn language_loader() -> &'static FluentLanguageLoader {
    LANGUAGE_LOADER.get().expect("Localization is not initialized")
}

pub(crate) mod helper {
    macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localization::language_loader(), $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localization::language_loader(), $message_id, $($args), *)
    }};
}
    pub(crate) use fl;
}
