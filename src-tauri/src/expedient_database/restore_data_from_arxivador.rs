use super::{Expedient, ExpedientDatabase, Order, OrderState, UtcDate};
use crate::database::Result;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::Write,
    fs::{self, create_dir_all, File},
    io::{self, BufReader},
    path::Path,
};
use tauri::api::path::document_dir;

#[derive(Deserialize)]
struct ArxiveExpedient {
    user: String,
    model: String,
    vin: String,
    matricula: String,
    date: String,
    process: u8,
    cos: String,
    id: usize,
}

type ArxiveChunk = Vec<(usize, ArxiveExpedient)>;

pub fn restore_data_from_arxivador(database: &mut ExpedientDatabase) -> Result<()> {
    if let Some(document_dir) = document_dir() {
        let arxivador_dir = document_dir.join("Arxivador");
        let mut chunks = Vec::<ArxiveChunk>::new();

        let mut expedient_ids = HashMap::new();

        for chunk_count in 1usize.. {
            let chunk_folder = arxivador_dir.join("data").join(chunk_count.to_string());
            if !chunk_folder.exists() {
                break;
            }
            let chunk_file = chunk_folder.join("expedients.json");
            let chunk_file_content = BufReader::new(File::open(chunk_file)?);
            let chunk = serde_json::from_reader(chunk_file_content).unwrap();
            chunks.push(chunk);

            // Read expedient folders
            for folder in chunk_folder.read_dir().unwrap() {
                if let Ok(dir) = folder {
                    if let Ok(id) = dir.file_name().to_str().unwrap().parse::<usize>() {
                        expedient_ids.insert(id, (dir.path(), 0));
                    }
                }
            }
        }

        let mut expedient_time_offset = 0;

        for chunk in chunks.iter() {
            for (_, expedient) in chunk {
                let parsed_date = expedient
                    .date
                    .split("/")
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                let day = parsed_date[0];
                let month = parsed_date[1];
                let year = parsed_date[2] as i32;

                let date_hash = UtcDate::ymdh(year, month, day, 8).date_hash();
                let date = UtcDate::from_hash(date_hash + expedient_time_offset);
                expedient_time_offset += 1;

                database.create_expedient(Expedient {
                    user: format_user(&expedient.user),
                    date,
                    vin: expedient.vin.to_uppercase(),
                    license_plate: format_license_plate(&expedient.matricula),
                    description: expedient.cos.clone(),
                    model: titlecase(&expedient.model),
                    orders: vec![Order {
                        date,
                        description: "".into(),
                        title: "Antic".into(),
                        state: OrderState::Done,
                    }],
                });

                if let Some(value) = expedient_ids.get_mut(&expedient.id) {
                    value.1 = date.date_hash();
                }
            }
        }
        database.save()?;

        let expedients_master_folder = document_dir.join("Archive").join("Expedients Folder");
        create_dir_all(&expedients_master_folder)?;

        for (_, (src_path, hash)) in expedient_ids {
            let dst_path = expedients_master_folder.join(&compress_date_hash(hash));
            println!("COPY: {:?} -> {:?}", &src_path, &dst_path);
            copy_content(src_path, dst_path)?;
        }
        //Expedients Folder
    }
    Ok(())
}

fn compress_date_hash(mut x: i64) -> String {
    // number to string (radix = 36)
    let mut result = vec![];

    loop {
        let m = x % 36;
        x = x / 36;
        result.push(std::char::from_digit(m as u32, 36).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect::<String>().to_uppercase()
}

fn format_license_plate(license: &String) -> String {
    let mut license = license.trim().to_uppercase();
    if license.len() > 4 {
        let chars = license.as_bytes();
        if (chars[4] as char).is_digit(10) && (chars[5] as char).is_alphabetic() {
            license.insert(5, ' ');
        }
    }
    license
}

fn format_user(user: &String) -> String {
    titlecase(user)
}

fn titlecase(text: &String) -> String {
    let mut formated_text = String::with_capacity(text.len());
    let mut first_letter_of_word = true;
    for char in text.trim().chars() {
        if char.is_alphabetic() {
            if first_letter_of_word {
                first_letter_of_word = false;
                formated_text
                    .write_str(&char.to_uppercase().to_string())
                    .unwrap();
            } else {
                formated_text.write_char(char).unwrap();
            }
        } else if char == ' ' {
            formated_text.write_char(char).unwrap();
            first_letter_of_word = true;
        } else {
            formated_text.write_char(char).unwrap();
            first_letter_of_word = false;
        }
    }
    formated_text.replace("Bmw", "BMW")
}

fn copy_content(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_content(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
