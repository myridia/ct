#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate ct_nox;
use ct::config::{get_config, save_config, save_last_dir};
use ct::icon::get_icon;
use ct_nox::ct_nox::{read_file, write_file};
use ct_nox::decrypt::decrypt;
use ct_nox::encrypt::encrypt;
use ct_nox::image_strip::{encode_to_image, encode_to_selected_image, decode_from_image};
use eframe::egui;
use eframe::egui::TextBuffer;
use eframe::egui::{ComboBox, IconData, Pos2, Vec2};

use egui::{Context, FontDefinitions};
use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::unic_langid;
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;
use std::collections::BTreeMap;
use std::ops::Range;
//use std::path::Path;
use unic_langid::LanguageIdentifier;
//use winres::*;
#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

fn main() -> Result<(), eframe::Error> {
    //println!("{:?}", config.language);
    /*
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        //res.set_icon("icon.ico")
            .set("InternalName", "TEST.EXE")
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        res.compile().unwrap();
    }
    */
    let (icon_rgba, icon_width, icon_height) = {
        let rgba = get_icon();
        (rgba, 64, 64)
    };

    let icon_data = IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    };

    //let icon_data = get_icon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(900.0, 750.0))
            .with_icon(icon_data),
        //icon_data: Some(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "CT",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(2.2);
            CT::configure_egui_fonts(&cc.egui_ctx);

            Box::new(CT::default())
        }),
    )
}

struct CT {
    loader: FluentLanguageLoader,
    text: String,
    picked_path: String,
    status_text: String,
    cursor1: usize,
    cursor2: usize,
    _password: String,
    last_dir: String,

    search_bar: bool,
    show_popup: bool,
    popup_position: Pos2,
    st: String,
    r: Range<usize>,
    panel_central: bool,
    panel_setting: bool,
    selected_language: String,
    language_map: BTreeMap<String, String>,
    open: String,
    _hide_password: bool,
    _search: String,
    search: String,
    save: String,
    copy: String,
    paste: String,
    cut: String,
    close: String,
    enter_text: String,
    status: String,
    about_us: String,
    exit: String,
    file: String,
    edit: String,
    settings: String,
    help: String,
    language: String,
    select_a_language: String,
    show_password: String,
    hide_password: String,
    password: String,
    lang_name: String,
}

//    fn new(_cc: &eframe::CreationContext<'_>) -> Self {

impl CT {
    fn pick_file_dialog(&mut self, dialog: rfd::FileDialog) -> Option<std::path::PathBuf> {
        let mut d = dialog;
        if !self.last_dir.is_empty() {
            d = d.set_directory(&self.last_dir);
        }
        let result = d.pick_file();
        if let Some(ref path) = result {
            if let Some(parent) = path.parent() {
                self.last_dir = parent.display().to_string();
                save_last_dir(&self.last_dir);
            }
        }
        result
    }

    fn save_file_dialog(&mut self, dialog: rfd::FileDialog) -> Option<std::path::PathBuf> {
        let mut d = dialog;
        if !self.last_dir.is_empty() {
            d = d.set_directory(&self.last_dir);
        }
        let result = d.save_file();
        if let Some(ref path) = result {
            if let Some(parent) = path.parent() {
                self.last_dir = parent.display().to_string();
                save_last_dir(&self.last_dir);
            }
        }
        result
    }

    pub fn configure_egui_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();

        //let base_dir = Path::new("assets");
        //let font_dir = base_dir.join("fonts");
        //let font_file_path = font_dir.join("noto-sans.ttf");

        fonts.font_data.insert(
            "noto_sans".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/noto-sans.ttf")),
        );

        fonts.font_data.insert(
            "noto_sans_cjk".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/noto-sans-cjk.otf")),
        );

        fonts.font_data.insert(
            "thai".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/thai.ttf")),
        );

        fonts.font_data.insert(
            "ethiopic".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/ethiopic.ttf")),
        );

        fonts.font_data.insert(
            "arabic".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/arabic.ttf")),
        );

        fonts.font_data.insert(
            "armenian".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/armenian.ttf")),
        );

        fonts.font_data.insert(
            "bengali".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/bengali.ttf")),
        );
        fonts.font_data.insert(
            "georgian".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/georgian.ttf")),
        );
        fonts.font_data.insert(
            "gujarati".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/gujarati.ttf")),
        );
        fonts.font_data.insert(
            "kannada".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/kannada.ttf")),
        );
        fonts.font_data.insert(
            "khmer".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/khmer.ttf")),
        );

        fonts.font_data.insert(
            "lao".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/lao.ttf")),
        );

        fonts.font_data.insert(
            "myammar".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/myammar.ttf")),
        );

        fonts.font_data.insert(
            "malayalam".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/malayalam.ttf")),
        );

        fonts.font_data.insert(
            "gurmukhi".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/gurmukhi.ttf")),
        );

                        fonts.font_data.insert(
            "sinhala".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/sinhala.ttf")),
        );

        fonts.font_data.insert(
            "tamil".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/tamil.ttf")),
        );

        fonts.font_data.insert(
            "telugu".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/telugu.ttf")),
        );

        fonts.font_data.insert(
            "hebrew".to_owned(),
            egui::FontData::from_static(include_bytes!("assets/fonts/hebrew.ttf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "noto_sans".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(1, "noto_sans_cjk".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(2, "thai".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(3, "ethiopic".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(4, "arabic".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(5, "armenian".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(6, "bengali".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(7, "georgian".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(8, "gujarati".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(9, "kannada".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(10, "khmer".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(11, "lao".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(12, "myammar".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(13, "malayalam".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(14, "sinhala".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(15, "tamil".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(16, "telugu".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(17, "hebrew".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(17, "gurmukhi".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "noto_sans".to_owned());

        ctx.set_fonts(fonts);
    }
}

impl Default for CT {
    fn default() -> Self {
        let config = get_config();
        let loader: FluentLanguageLoader = fluent_language_loader!();

        //let _result = i18n_embed::select(&loader, &Localizations, &requested_languages);

        let new_code = &config.language;
        let new_lang_id: LanguageIdentifier = new_code.parse().unwrap();
        let mut new_languages: Vec<LanguageIdentifier> = Vec::new();
        new_languages.push(new_lang_id);
        let _result = i18n_embed::select(&loader, &Localizations, &new_languages);
        //let ftl: Vec<String> = env!("ftl").split(',').map(|s| s.to_string()).collect();
        let mut language_map: BTreeMap<String, String> = BTreeMap::new();

        // Populate with the transformed data:
        language_map.insert("English English".to_string(), "en-US".to_string());
        language_map.insert("Thai ไทย".to_string(), "th-TH".to_string());
        language_map.insert("Afrikaans Afrikaans".to_string(), "af-ZA".to_string());
        language_map.insert("Albanian Shqip".to_string(), "sq-AL".to_string());
        language_map.insert("Amharic አማርኛ".to_string(), "am-ET".to_string());
        language_map.insert("Arabic العربية".to_string(), "ar-SA".to_string());
        language_map.insert("Armenian Հայերեն".to_string(), "hy-AM".to_string());
        language_map.insert(
            "Azerbaijani Azərbaycan dili".to_string(),
            "az-AZ".to_string(),
        );
        language_map.insert("Basque Euskara".to_string(), "eu-ES".to_string());
        language_map.insert("Belarusian Беларуская".to_string(), "be-BY".to_string());
        language_map.insert("Bengali বাংলা".to_string(), "bn-BD".to_string());
        language_map.insert("Bosnian Bosanski".to_string(), "bs-BA".to_string());
        language_map.insert("Bulgarian Български".to_string(), "bg-BG".to_string());
        language_map.insert("Catalan Català".to_string(), "ca-ES".to_string());
        language_map.insert("Chichewa Chinyanja".to_string(), "ny-MW".to_string()); // Also Nyanja
        language_map.insert("Corsican Corsu".to_string(), "co-FR".to_string());
        language_map.insert("Croatian Hrvatski".to_string(), "hr-HR".to_string());
        language_map.insert("Czech Čeština".to_string(), "cs-CZ".to_string());
        language_map.insert("Danish Dansk".to_string(), "da-DK".to_string());
        language_map.insert("Dutch Nederlands".to_string(), "nl-NL".to_string());
        language_map.insert("Esperanto Esperanto".to_string(), "eo".to_string());
        language_map.insert("Estonian Eesti keel".to_string(), "et-EE".to_string());
        language_map.insert("Filipino Tagalog".to_string(), "tl-PH".to_string());
        language_map.insert("Finnish Suomi".to_string(), "fi-FI".to_string());
        language_map.insert("French Français".to_string(), "fr-FR".to_string());
        language_map.insert("Frisian Frysk".to_string(), "fy-NL".to_string());
        language_map.insert("Galician Galego".to_string(), "gl-ES".to_string());
        language_map.insert("Georgian ქართული".to_string(), "ka-GE".to_string());
        language_map.insert("German Deutsch".to_string(), "de-DE".to_string());
        language_map.insert("Greek Ελληνικά".to_string(), "el-GR".to_string());
        language_map.insert("Gujarati ગુજરાતી".to_string(), "gu-IN".to_string());
        language_map.insert(
            "Haitian Creole Kreyòl ayisyen".to_string(),
            "ht-HT".to_string(),
        );
        language_map.insert("Hausa Hausa".to_string(), "ha-NG".to_string());
        language_map.insert("Hawaiian ʻŌlelo Hawaiʻi".to_string(), "haw-US".to_string());
        language_map.insert("Hindi हिन्दी".to_string(), "hi-IN".to_string());
        language_map.insert("Hmong Hmong".to_string(), "hmn".to_string()); // Generic, as region isn't specified
        language_map.insert("Hungarian Magyar".to_string(), "hu-HU".to_string());
        language_map.insert("Igbo Igbo".to_string(), "ig-NG".to_string());
        language_map.insert("Irish Gaeilge".to_string(), "ga-IE".to_string());
        language_map.insert("Italian Italiano".to_string(), "it-IT".to_string());
        language_map.insert("Japanese 日本語".to_string(), "ja-JP".to_string());
        language_map.insert("Kannada ಕನ್ನಡ".to_string(), "kn-IN".to_string());
        language_map.insert("Kazakh Қазақ тілі".to_string(), "kk-KZ".to_string());
        language_map.insert("Khmer Khmer".to_string(), "km-KH".to_string());
        language_map.insert("Korean 한국어".to_string(), "ko-KR".to_string());
        language_map.insert("Kurdish Kurdî".to_string(), "ku-TR".to_string()); // Often Kurdish Sorani or Kurmanji
        language_map.insert("Kyrgyz Кыргызча".to_string(), "ky-KG".to_string());
        language_map.insert("Lao ລາວ".to_string(), "lo-LA".to_string());
        language_map.insert("Latin Latina".to_string(), "la".to_string()); // Generic Latin
        language_map.insert("Latvian Latviešu valoda".to_string(), "lv-LV".to_string());
        language_map.insert("Lithuanian Lietuvių kalba".to_string(), "lt-LT".to_string());
        language_map.insert(
            "Luxembourgish Lëtzebuergesch".to_string(),
            "lb-LU".to_string(),
        );
        language_map.insert("Macedonian Македонски".to_string(), "mk-MK".to_string());
        language_map.insert("Malagasy Malagasy".to_string(), "mg-MG".to_string());
        language_map.insert("Malay Bahasa Melayu".to_string(), "ms-MY".to_string());
        language_map.insert("Malayalam മലയാളം".to_string(), "ml-IN".to_string());
        language_map.insert("Maltese Malti".to_string(), "mt-MT".to_string());
        language_map.insert("Maori Te Reo Māori".to_string(), "mi-NZ".to_string());
        language_map.insert("Marathi मराठी".to_string(), "mr-IN".to_string());
        language_map.insert("Mongolian Монгол".to_string(), "mn-MN".to_string());
        language_map.insert("Myanmar မြန်မာ".to_string(), "my-MM".to_string());
        language_map.insert("Nepali नेपाली".to_string(), "ne-NP".to_string());
        language_map.insert("Norwegian Norsk".to_string(), "no-NO".to_string());
        language_map.insert("Pashto پښتو".to_string(), "ps-AF".to_string());
        language_map.insert("Persian فارسی".to_string(), "fa-IR".to_string());
        language_map.insert("Polish Polski".to_string(), "pl-PL".to_string());
        language_map.insert("Portuguese Português".to_string(), "pt-PT".to_string());
        language_map.insert("Punjabi ਪੰਜਾਬੀ".to_string(), "pa-IN".to_string());
        language_map.insert("Romanian Română".to_string(), "ro-RO".to_string());
        language_map.insert("Russian Русский".to_string(), "ru-RU".to_string());
        language_map.insert("Samoan Gagana fa'a Sāmoa".to_string(), "sm-WS".to_string());
        language_map.insert("Scottish Gaelic Gàidhlig".to_string(), "gd-GB".to_string());
        language_map.insert("Serbian Srpski".to_string(), "sr-RS".to_string());
        language_map.insert("Sesotho Sesotho".to_string(), "st-ZA".to_string());
        language_map.insert("Shona Chishona".to_string(), "sn-ZW".to_string());
        language_map.insert("Sindhi سنڌي".to_string(), "sd-PK".to_string());
        language_map.insert("Sinhala සිංහල".to_string(), "si-LK".to_string());
        language_map.insert("Slovak Slovenčina".to_string(), "sk-SK".to_string());
        language_map.insert("Slovenian Slovenščina".to_string(), "sl-SI".to_string());
        language_map.insert("Somali Soomaali".to_string(), "so-SO".to_string());
        language_map.insert("Spanish Español".to_string(), "es-ES".to_string());
        language_map.insert("Sundanese Basa Sunda".to_string(), "su-ID".to_string());
        language_map.insert("Swahili Kiswahili".to_string(), "sw-TZ".to_string());
        language_map.insert("Swedish Svenska".to_string(), "sv-SE".to_string());
        language_map.insert("Tajik Тоҷикӣ".to_string(), "tg-TJ".to_string());
        language_map.insert("Tamil தமிழ்".to_string(), "ta-IN".to_string());
        language_map.insert("Telugu తెలుగు".to_string(), "te-IN".to_string());
        language_map.insert("Turkish Türkçe".to_string(), "tr-TR".to_string());
        language_map.insert("Ukrainian Українська".to_string(), "uk-UA".to_string());
        language_map.insert("Urdu اردو".to_string(), "ur-PK".to_string());
        language_map.insert("Uzbek Oʻzbek tili".to_string(), "uz-UZ".to_string());
        language_map.insert("Vietnamese Tiếng Việt".to_string(), "vi-VN".to_string());
        language_map.insert("Welsh Cymraeg".to_string(), "cy-GB".to_string());
        language_map.insert("Xhosa isiXhosa".to_string(), "xh-ZA".to_string());
        language_map.insert("Yiddish ייִדיש".to_string(), "yi".to_string()); // Generic Yiddish
        language_map.insert("Yoruba Yorùbá".to_string(), "yo-NG".to_string());
        language_map.insert("Zulu isiZulu".to_string(), "zu-ZA".to_string());

        let open = fl!(loader, "open");
        let save = fl!(loader, "save");
        let copy = fl!(loader, "copy");
        let paste = fl!(loader, "paste");
        let cut = fl!(loader, "cut");
        let search = fl!(loader, "search");
        let close = fl!(loader, "close");
        let enter_text = fl!(loader, "enter_text");
        let status = fl!(loader, "status");
        let show_password = fl!(loader, "show_password");
        let hide_password = fl!(loader, "hide_password");
        let password = fl!(loader, "password");
        let about_us = fl!(loader, "about_us");
        let exit = fl!(loader, "exit");
        let file = fl!(loader, "file");
        let edit = fl!(loader, "edit");

        let settings = fl!(loader, "settings");
        let help = fl!(loader, "help");
        let language = fl!(loader, "languages");
        let select_a_language = fl!(loader, "select_a_language");

        CT {
            loader: loader,
            text: "".to_string(),
            picked_path: "".to_string(),
            status_text: "".to_string(),
            lang_name: "".to_string(),
            cursor1: 0,
            cursor2: 0,
            _password: "".to_string(),
            st: "".to_string(),
            r: 0..0,
            _hide_password: false,
            search_bar: false,
            _search: "".to_string(),
            show_popup: false,
            popup_position: Pos2 { x: 0.0, y: 0.0 },
            panel_central: true,
            panel_setting: false,
            selected_language: config.language,
            language_map: language_map,
            open: open,
            search: search,
            save: save,
            copy: copy,
            paste: paste,
            cut: cut,
            close: close,
            enter_text: enter_text,
            status: status,
            about_us: about_us,
            exit: exit,
            file: file,
            edit: edit,
            settings: settings,
            help: help,
            language: language,
            select_a_language: select_a_language,
            show_password: show_password,
            hide_password: hide_password,
            password: password,
            last_dir: config.last_dir,
        }
    }
}

impl eframe::App for CT {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.panel_central == false && self.panel_setting == true {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(20.0);
                ComboBox::new(
                    "language",
                    format!("{}  Select a Language", self.select_a_language),
                )
                .selected_text(&self.selected_language)
                .show_ui(ui, |ui| {
                    for (i, _name) in &self.language_map {
                        if ui.selectable_label(false, i).clicked() {
                            self.selected_language = self.language_map[&i.clone()].clone();
                            self.lang_name = i.to_string();
                            save_config(&self.selected_language);

                            let new_code = self.selected_language.clone();
                            let new_lang_id: LanguageIdentifier = new_code.parse().unwrap();
                            let mut new_languages: Vec<LanguageIdentifier> = Vec::new();
                            new_languages.push(new_lang_id);
                            let _result =
                                i18n_embed::select(&self.loader, &Localizations, &new_languages);
                            self.open = fl!(&self.loader, "open");
                            self.password = fl!(&self.loader, "password");
                            self.save = fl!(&self.loader, "save");
                            self.copy = fl!(&self.loader, "copy");
                            self.paste = fl!(&self.loader, "paste");
                            self.cut = fl!(&self.loader, "cut");
                            self.search = fl!(&self.loader, "search");
                            self.close = fl!(&self.loader, "close");
                            self.enter_text = fl!(&self.loader, "enter_text");
                            self.status = fl!(&self.loader, "status");
                            self.show_password = fl!(&self.loader, "open");
                            self.hide_password = fl!(&self.loader, "open");
                            self.about_us = fl!(&self.loader, "about_us");
                            self.exit = fl!(&self.loader, "exit");
                            self.file = fl!(&self.loader, "file");
                            self.edit = fl!(&self.loader, "edit");
                            self.settings = fl!(&self.loader, "settings");
                            self.help = fl!(&self.loader, "help");
                            self.language = fl!(&self.loader, "languages");
                            self.select_a_language = fl!(&self.loader, "select_a_language");

                            println!("{}", fl!(self.loader, "open"));
                        }
                    }
                });

                ui.add_space(20.0);
                ui.label(format!("You selected: {}", &self.lang_name));
            });
        } else if self.panel_central == true && self.panel_setting == false {
            egui::CentralPanel::default().show(ctx, |ui| {
                let r = get_char_range(self.cursor1, self.cursor2);
                let stl = self.text.char_range(r.clone()).to_string();
                //println!("{:?}", r.);

                if stl.len() > 0 {
                    self.st = stl;
                    self.r = r;
                }

                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    let spacing = ui.spacing().item_spacing.x;
                    let available_width = ui.available_width() - (spacing * 2.0);
                    let button_width = (available_width / 100.00) * 25.00;
                    let password_width = (available_width / 100.00) * 75.00;
                    let button_height = 20.0;
                    let button_size = egui::Vec2::new(button_width, button_height);

                    let _password = ui.add(
                        egui::TextEdit::singleline(&mut self._password)
                            .hint_text(&self.password)
                            .desired_width(password_width)
                            .password(!self._hide_password),
                    );

                    let button_text = if self._hide_password {
                        &self.hide_password
                    } else {
                        &self.show_password
                    };
                    if ui
                        .add(egui::Button::new(button_text).min_size(button_size))
                        .clicked()
                    {
                        self._hide_password = !self._hide_password;
                    }
                });

                ui.horizontal(|ui| {
                    let num_buttons = 10.0;
                    let spacing = ui.spacing().item_spacing.x;
                    let total_spacing = spacing * (num_buttons - 1.0);

                    let available_width = ui.available_width();
                    let button_width = (available_width - total_spacing) / num_buttons;
                    let button_height = 20.0;
                    let button_size = egui::Vec2::new(button_width, button_height);

                    if ui
                        .add(egui::Button::new(&self.open).min_size(button_size))
                        .clicked()
                    {
                        if let Some(path) = self.pick_file_dialog(rfd::FileDialog::new()) {
                            self.picked_path = path.display().to_string();
                            let ct = read_file(&self.picked_path.clone());
                            self.text = decrypt(&ct, &self._password);
                        }
                    }

                    if ui
                        .add(egui::Button::new(&self.save).min_size(button_size))
                        .clicked()
                    {
                        if let Some(path) = self.save_file_dialog(rfd::FileDialog::new()) {
                            self.picked_path = ensure_extension(&path.display().to_string(), ".ct");
                            let ct = encrypt(&self.text, &self._password);
                            let _x = write_file(&self.picked_path.clone(), &ct);
                        }
                    }

                    if ui
                        .add(egui::Button::new("Export Image").min_size(button_size))
                        .clicked()
                    {
                        if let Some(path) = self.save_file_dialog(
                            rfd::FileDialog::new().add_filter("PNG", &["png"]),
                        ) {
                            let out = ensure_extension(&path.display().to_string(), ".png");
                            let ct = encrypt(&self.text, &self._password);
                            let _ = encode_to_image(&ct, &out);
                        }
                    }

                    if ui
                        .add(egui::Button::new("Import Image").min_size(button_size))
                        .clicked()
                    {
                        if let Some(path) = self.pick_file_dialog(
                            rfd::FileDialog::new().add_filter("PNG", &["png"]),
                        ) {
                            if let Ok(ct) = decode_from_image(&path.display().to_string()) {
                                self.text = decrypt(&ct, &self._password);
                            }
                        }
                    }

                    if ui
                        .add(egui::Button::new("Export Selected").min_size(button_size))
                        .clicked()
                    {
                        if let Some(bg) = self.pick_file_dialog(
                            rfd::FileDialog::new().add_filter("PNG", &["png"]),
                        ) {
                            if let Some(out) = self.save_file_dialog(
                                rfd::FileDialog::new().add_filter("PNG", &["png"]),
                            ) {
                                let out_path = ensure_extension(&out.display().to_string(), ".png");
                                let ct = encrypt(&self.text, &self._password);
                                let _ = encode_to_selected_image(
                                    &ct,
                                    &bg.display().to_string(),
                                    &out_path,
                                );
                            }
                        }
                    }

                    if ui
                        .add(egui::Button::new(&self.cut).min_size(button_size))
                        .clicked()
                    {
                        let r = get_char_range(self.cursor1, self.cursor2);
                        let st = self.text.char_range(r.clone());
                        ui.output_mut(|o| o.copied_text = st.to_string());
                        self.text.delete_char_range(r.clone());
                    }

                    if ui
                        .add(egui::Button::new(&self.copy).min_size(button_size))
                        .clicked()
                    {
                        let r = get_char_range(self.cursor1, self.cursor2);
                        let st = self.text.char_range(r.clone());
                        ui.output_mut(|o| o.copied_text = st.to_string());
                    }
                    if ui
                        .add(egui::Button::new(&self.paste).min_size(button_size))
                        .clicked()
                    {
                        let txt = cli_clipboard::get_contents().unwrap();
                        let r = get_char_range(self.cursor1, self.cursor2);
                        self.text.insert_text(&txt, r.start);
                    }
                    if ui
                        .add(egui::Button::new(&self.search).min_size(button_size))
                        .clicked()
                    {
                        self.search_bar = !self.search_bar;
                    }

                    if ui
                        .add(egui::Button::new(&self.close).min_size(button_size))
                        .clicked()
                    {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                if self.search_bar {
                    ui.horizontal(|ui| {
                        let _search = ui.add(
                            egui::TextEdit::singleline(&mut self._search)
                                .hint_text(&self.search)
                                .desired_width(f32::INFINITY),
                        );
                    });
                }

                ui.add_space(2.0);
                let _scroll = egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let target_word: &str = self._search.as_str();
                        if let Some(pos) = string.find(target_word) {
                            if !target_word.is_empty() {
                                layout_job.append(&string[..pos], 0.0, egui::TextFormat::default());

                                let red_color = egui::Color32::RED;
                                let color_format = egui::TextFormat {
                                    color: red_color,
                                    ..Default::default()
                                };
                                layout_job.append(
                                    &string[pos..pos + target_word.len()],
                                    0.0,
                                    color_format,
                                );

                                layout_job.append(
                                    &string[pos + target_word.len()..],
                                    0.0,
                                    egui::TextFormat::default(),
                                );
                            } else {
                                layout_job.append(string, 0.0, egui::TextFormat::default());
                            }
                        } else {
                            layout_job.append(string, 0.0, egui::TextFormat::default());
                        }

                        layout_job.wrap.max_width = wrap_width;

                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    let textedit = egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                        .hint_text(&self.enter_text)
                        .layouter(&mut layouter);
                    let response = ui.add_sized(ui.available_size(), textedit);
                    //https://docs.rs/egui/0.21.0/egui/struct.Response.html#method.hovered
                    let resp_id = response.id;

                    if let Some(state) = egui::TextEdit::load_state(ui.ctx(), resp_id) {
                        //if let Some(ccursor) = state.ccursor_range() {
                        if let Some(ccursor) = state.cursor.char_range() {
                            self.cursor1 = ccursor.secondary.index;
                            self.cursor2 = ccursor.primary.index;
                        }
                    }

                    if response.clicked_by(egui::PointerButton::Secondary) {
                        self.show_popup = true;
                        self.popup_position = response.interact_pointer_pos().unwrap_or(Pos2::ZERO); // Store click position
                    }
                });
            });
        };
        if self.show_popup {
            let popup_id = egui::Id::new("my_popup");
            egui::Area::new(popup_id)
                .fixed_pos(self.popup_position)
                .show(ctx, |ui| {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        if ui.button(&self.copy).clicked() {
                            ui.output_mut(|o| o.copied_text = self.st.to_string());
                            self.show_popup = false;
                        }

                        if ui.button(&self.paste).clicked() {
                            let txt = cli_clipboard::get_contents().unwrap();
                            let r = get_char_range(self.cursor1, self.cursor2);
                            self.text.insert_text(&txt, r.start);
                            self.show_popup = false;
                        }
                        if ui.button(&self.cut).clicked() {
                            ui.output_mut(|o| o.copied_text = self.st.to_string());
                            self.text.delete_char_range(self.r.clone());
                            self.show_popup = false;
                        }

                        if ui.button(&self.close).clicked() {
                            self.show_popup = false;
                        }
                    });

                    if ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary)) {
                        self.show_popup = false;
                    }
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(self.file.clone(), |ui| {
                    if ui.button("Open CT File").clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        if let Some(path) = self.pick_file_dialog(rfd::FileDialog::new()) {
                            self.picked_path = path.display().to_string();
                            let ct = read_file(&self.picked_path.clone());
                            self.text = decrypt(&ct, &self._password);
                        }

                        ui.close_menu();
                    }
                    if ui.button("Save CT File").clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        if let Some(path) = self.save_file_dialog(rfd::FileDialog::new()) {
                            self.picked_path = ensure_extension(&path.display().to_string(), ".ct");
                            let ct = encrypt(&self.text, &self._password);
                            let _x = write_file(&self.picked_path.clone(), &ct);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Open Text File").clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        if let Some(path) = self.pick_file_dialog(rfd::FileDialog::new()) {
                            self.picked_path = path.display().to_string();
                            self.text = read_file(&self.picked_path.clone());
                        }

                        ui.close_menu();
                    }
                    if ui.button("Export as Image").clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        if let Some(path) = self.save_file_dialog(
                            rfd::FileDialog::new().add_filter("PNG", &["png"]),
                        ) {
                            let out = ensure_extension(&path.display().to_string(), ".png");
                            let ct = encrypt(&self.text, &self._password);
                            let _ = encode_to_image(&ct, &out);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Import from Image").clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        if let Some(path) = self.pick_file_dialog(
                            rfd::FileDialog::new().add_filter("PNG", &["png"]),
                        ) {
                            if let Ok(ct) = decode_from_image(&path.display().to_string()) {
                                self.text = decrypt(&ct, &self._password);
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Export as Selected Image").clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        if let Some(bg) = self.pick_file_dialog(
                            rfd::FileDialog::new().add_filter("PNG", &["png"]),
                        ) {
                            if let Some(out) = self.save_file_dialog(
                                rfd::FileDialog::new().add_filter("PNG", &["png"]),
                            ) {
                                let out_path = ensure_extension(&out.display().to_string(), ".png");
                                let ct = encrypt(&self.text, &self._password);
                                let _ = encode_to_selected_image(
                                    &ct,
                                    &bg.display().to_string(),
                                    &out_path,
                                );
                            }
                        }
                        ui.close_menu();
                    }
                });
                ui.menu_button(self.edit.clone(), |ui| {
                    if ui.button(&self.copy).clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;

                        let r = get_char_range(self.cursor1, self.cursor2);
                        let st = self.text.char_range(r.clone());
                        ui.output_mut(|o| o.copied_text = st.to_string());

                        ui.close_menu();
                    }
                    if ui.button(&self.paste).clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        let txt = cli_clipboard::get_contents().unwrap();
                        let r = get_char_range(self.cursor1, self.cursor2);
                        self.text.insert_text(&txt, r.start);
                        ui.close_menu();
                    }
                    if ui.button(&self.cut).clicked() {
                        self.panel_central = true;
                        self.panel_setting = false;
                        ui.output_mut(|o| o.copied_text = self.st.to_string());
                        self.text.delete_char_range(self.r.clone());
                        self.show_popup = false;
                        ui.close_menu();
                    }
                });
                ui.menu_button(self.settings.clone(), |ui| {
                    if ui.button(&self.language).clicked() {
                        self.panel_central = false;
                        self.panel_setting = true;

                        ui.close_menu();
                    }
                });
                ui.menu_button(self.about_us.clone(), |ui| {
                    if ui.button(&self.help).clicked() {
                        ui.close_menu();
                    }
                    if ui.button(&self.about_us).clicked() {
                        ui.close_menu();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{0}: {1}", &self.status, self.status_text));
                ui.separator();
                //ui.add(egui::ProgressBar::new(self.progress).show_percentage());
            });
        });
    }
}

fn ensure_extension(path: &str, ext: &str) -> String {
    match path.rfind('.') {
        Some(pos) => {
            let existing = &path[pos..];
            if existing.eq_ignore_ascii_case(ext) {
                path.to_string()
            } else {
                format!("{}{}", path, ext)
            }
        }
        None => format!("{}{}", path, ext),
    }
}

fn get_char_range(c1: usize, c2: usize) -> std::ops::Range<usize> {
    //https://docs.rs/egui/latest/egui/widgets/text_edit/trait.TextBuffer.html#method.char_range
    let mut a = c1;
    let mut b = c2;
    if a > b {
        a = c2;
        b = c1;
    }
    let r = std::ops::Range { start: a, end: b };
    return r;
}
