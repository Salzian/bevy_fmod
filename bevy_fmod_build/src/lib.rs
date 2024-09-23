use libfmod::ffi::FMOD_STUDIO_LOAD_BANK_NORMAL;
use libfmod::EventDescription;
use quote::quote;
use rayon::prelude::*;
use std::fs::canonicalize;

/// Build metadata of your FMOD project at compile-time and use strong references to events.
pub fn build_metadata(bank_files: &[&str]) {
    // Convert the bank_files array to paths
    let bank_paths = bank_files
        .par_iter()
        .map(|bank_file| canonicalize(bank_file).unwrap());

    // Init FMOD studio
    let studio = libfmod::Studio::create().unwrap();

    // Load all banks
    bank_paths.for_each(|bank_path| {
        studio
            .load_bank_file(bank_path.to_str().unwrap(), FMOD_STUDIO_LOAD_BANK_NORMAL)
            .unwrap();
    });

    let banks = studio
        .get_bank_list(studio.get_bank_count().unwrap())
        .unwrap();

    banks
        .par_iter()
        .map(|bank| {
            (
                bank,
                bank.get_event_list(bank.get_event_count().unwrap())
                    .unwrap(),
            )
        })
        .for_each(|(bank, events)| {
            let events_code = generate_event_references_code(&events);
            println!("{events_code}")
        });
}

fn generate_event_references_code(events: &Vec<EventDescription>) -> proc_macro2::TokenStream {
    let mut events: Vec<(String, String)> = events
        .iter()
        .map(|event| {
            let path = event.get_path().unwrap();
            let name = path
                .trim_start_matches("event:")
                .replace("/", "_")
                .to_uppercase();

            (name, path)
        })
        .collect();

    events.sort_by_cached_key(|(name, _)| name.clone());

    let events_code_snippets = events.iter().map(|(name, path)| {
        quote! {
            const #name: &'static str = #path;
        }
    });

    quote! {
        #(#events_code_snippets);*
    }
}
