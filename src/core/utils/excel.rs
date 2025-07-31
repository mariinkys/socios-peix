use leptos::prelude::*;

use crate::core::models::member::MemberWithInterests;

#[server(GenMembersExcel, "/api/generate/members-excel")]
pub async fn generate_members_excel(
    model: Vec<MemberWithInterests>,
) -> Result<String, ServerFnError> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use chrono::Datelike;
    use rust_xlsxwriter::*;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold_format = Format::new().set_bold();
    let date_format = Format::new().set_num_format("dd/mm/yyyy");
    let text_wrap_format = Format::new().set_text_wrap();

    let headers = vec![
        "ID",
        "Nombre",
        "Primer Apellido",
        "Segundo Apellido",
        "Correo Electrónico",
        "Fecha de Nacimiento",
        "Teléfono",
        "País",
        "Género",
        "Notas",
        "Intereses",
        "Creado en",
        "Actualizado en",
    ];

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_with_format(0, col as u16, *header, &bold_format)?;
    }

    for (row_idx, member_with_interests) in model.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        let m = &member_with_interests.member;

        let interests_str = member_with_interests
            .interests
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        worksheet.write(row, 0, m.id.unwrap_or_default())?;
        worksheet.write(row, 1, &m.name)?;
        worksheet.write(row, 2, &m.surname)?;
        worksheet.write(row, 3, &m.second_surname)?;
        worksheet.write(row, 4, &m.email)?;

        if let Some(bd) = m.birthdate {
            let excel_date = ExcelDateTime::from_ymd(
                bd.year().try_into().unwrap_or_default(),
                bd.month() as u8,
                bd.day() as u8,
            )?;
            worksheet.write_with_format(row, 5, &excel_date, &date_format)?;
        } else {
            worksheet.write(row, 5, "")?;
        }

        worksheet.write(row, 6, &m.phone)?;
        worksheet.write(row, 7, m.country.to_string())?;
        worksheet.write(row, 8, m.gender.to_string())?;
        worksheet.write(row, 9, &m.notes)?;
        worksheet.write_with_format(row, 10, &interests_str, &text_wrap_format)?;

        if let Some(created) = m.created_at {
            let excel_date = ExcelDateTime::from_ymd(
                created.year().try_into().unwrap_or_default(),
                created.month() as u8,
                created.day() as u8,
            )?;
            worksheet.write_with_format(row, 11, &excel_date, &date_format)?;
        } else {
            worksheet.write(row, 11, "")?;
        }

        // Updated_at
        if let Some(updated) = m.updated_at {
            let excel_date = ExcelDateTime::from_ymd(
                updated.year().try_into().unwrap_or_default(),
                updated.month() as u8,
                updated.day() as u8,
            )?;
            worksheet.write_with_format(row, 12, &excel_date, &date_format)?;
        } else {
            worksheet.write(row, 12, "")?;
        }
    }

    for col in 0..headers.len() {
        worksheet.set_column_width(col as u16, 20)?;
    }

    match workbook.save_to_buffer() {
        Ok(bytes) => Ok(STANDARD.encode(&bytes)),
        Err(err) => {
            eprintln!("Error generating Excel: {err}");
            Err(ServerFnError::new(format!("Error: {err}")))
        }
    }
}
