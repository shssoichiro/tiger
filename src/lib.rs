//! This library forms the bulk of the -tiger family of validators: `ck3-tiger`, `vic3-tiger`, and
//! `imperator-tiger`. Each executable is a small wrapper around the functions in this library that
//! start and perform validation.

#[cfg(all(
    feature = "ck3",
    feature = "vic3",
    feature = "imperator",
    feature = "eu5",
    feature = "hoi4",
    not(doc)
))]
compile_error!(
    "features \"ck3\", \"vic3\", \"imperator\", \"eu5\", and \"hoi4\" cannot be enabled at the same time"
);

#[cfg(all(
    not(feature = "ck3"),
    not(feature = "vic3"),
    not(feature = "imperator"),
    not(feature = "eu5"),
    not(feature = "hoi4")
))]
compile_error!(
    "exactly one of the features \"ck3\", \"vic3\", \"imperator\", \"eu5\", \"hoi4\" must be enabled"
);

pub use crate::config_load::validate_config_file;
pub use crate::everything::Everything;
pub use crate::fileset::FileKind;
pub use crate::game::Game;
pub use crate::helpers::{TigerHashMap, TigerHashSet};
pub use crate::item::Item;
pub use crate::launcher_settings::get_version_from_launcher;
#[cfg(any(feature = "vic3", feature = "eu5"))]
pub use crate::mod_metadata::ModMetadata;
#[cfg(any(feature = "ck3", feature = "imperator", feature = "hoi4"))]
pub use crate::modfile::ModFile;
pub use crate::report::{
    Confidence, LogReportMetadata, LogReportPointers, PointedMessage, Severity,
    add_loaded_mod_root, disable_ansi_colors, emit_reports, log, set_output_style,
    set_show_loaded_mods, set_show_vanilla, suppress_from_json, take_reports,
};
pub use crate::token::{Loc, Token};

#[cfg(feature = "internal_benches")]
mod benches;

#[cfg(feature = "ck3")]
mod ck3;
#[cfg(feature = "eu5")]
mod eu5;
#[cfg(feature = "hoi4")]
mod hoi4;
#[cfg(feature = "imperator")]
mod imperator;
#[cfg(feature = "vic3")]
pub mod vic3;

pub mod block;
pub mod config_load;
pub mod context;
pub mod data;
pub mod datacontext;
pub mod datatype;
pub mod date;
pub mod db;
pub mod dds;
pub mod defines;
pub mod desc;
pub mod effect;
#[cfg(feature = "jomini")]
pub mod effect_validation;
pub mod everything;
pub mod fileset;
pub mod game;
pub mod gui;
pub mod helpers;
pub mod item;
pub mod launcher_settings;
pub mod lowercase;
pub mod macros;
#[cfg(any(feature = "vic3", feature = "eu5"))]
pub mod mod_metadata;
#[cfg(any(feature = "ck3", feature = "imperator", feature = "hoi4"))]
pub mod modfile;
pub mod modif;
pub mod on_action;
pub mod parse;
pub mod pathtable;
pub mod pdxfile;
pub mod report;
pub mod rivers;
pub mod scopes;
#[cfg(feature = "jomini")]
pub mod script_value;
pub mod special_tokens;
pub mod token;
pub mod tooltipped;
pub mod trigger;
pub mod util;
pub mod validate;
pub mod validator;
#[cfg(feature = "jomini")]
pub mod variable_scopes;
pub mod variables;
