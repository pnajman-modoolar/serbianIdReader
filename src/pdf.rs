extern crate printpdf;
use printpdf::*;
use std::convert::From;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::idreader::reader::*;
use chrono;


fn format_date(date_str: &str) -> String {
    let date_value = chrono::NaiveDate::parse_from_str(date_str, "%d%m%Y").unwrap();
    date_value.format("%d.%m.%Y.").to_string()

}

fn add_line(x: f64, y: f64, current_layer: &PdfLayerReference) {
    let points1 = vec![
    (Point::new(Mm(x), Mm(y)), false),
    (Point::new(Mm(x+176.0), Mm(y)), false)
    ];
    
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };
    
    let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

    current_layer.set_outline_color(outline_color);
    current_layer.set_outline_thickness(1.0);
    current_layer.add_shape(line1);

}

fn add_image(x: f64, y: f64, buffer: &[u8], current_layer: &PdfLayerReference) -> Result<(), String> {
    let points1 = vec![
        (Point::new(Mm(x + 0.0), Mm(259.0)), false),
        (Point::new(Mm(x + 42.0), Mm(259.0)), false),
        (Point::new(Mm(x + 42.0), Mm(203.0)), false),
        (Point::new(Mm(x + 0.0), Mm(203.0)), false),
    ];

    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(20);

    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_color(outline_color);
    current_layer.set_outline_thickness(0.1);

    current_layer.add_shape(line1);


    let dyn_image = image_crate::load_from_memory(buffer).or_else(|err| return Err(err.to_string()))?;
    let ximage = ImageXObject::from_dynamic_image(&dyn_image);
    let image = Image::from(ximage);
    image.add_to_layer(
        current_layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(x)),
            translate_y: Some(Mm(y)),
            rotate: None,
            scale_x: Some(2.06),
            scale_y: Some(2.06),
            dpi: None,
        },
    );
    Ok(())
}

fn add_text(x:f64, y:f64, text: &str, font_size: f64, font: &IndirectFontRef, current_layer: &PdfLayerReference) {
    current_layer.begin_text_section();

    current_layer.set_font(&font, font_size);
    current_layer.set_text_cursor(Mm(x), Mm(y));
    current_layer.set_line_height(5.0);
    current_layer.set_word_spacing(5.0);
    current_layer.set_character_spacing(0.3);
    current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
    current_layer.write_text(text.clone(), &font);
    current_layer.add_line_break();

    current_layer.end_text_section();
}

pub fn copy_font() {
    let font_bytes = include_bytes!("FreeSans.ttf");
    if !std::path::Path::new("/tmp/FreeSans.ttf").exists() {
        let mut file = File::create("/tmp/FreeSans.ttf").expect("failed to open file");
        file.write_all(font_bytes).expect("Failed to write the file");
    }

}

pub fn topdf(personal_id: &PersonalId, path: &str) -> Result<(), String>{
    let (doc, page1, layer1) =
        PdfDocument::new("Podaci licne karte", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let left_margin = 18.0;
    let data_margin = 54.0;

    let font_file = match File::open("/tmp/FreeSans.ttf") {
        Ok(file) => file,
        Err(str) => return Err(str.to_string())
    };
    let font2 = match doc.add_external_font(font_file) {
        Ok(font) => font,
        Err(str) => return Err(str.to_string())
    };


    let empty_item = PersonalIdItem::default();

    let surname = &personal_id.personal.get(&PersonalIdTag::Surname).unwrap_or(&empty_item).value;
    let name = &personal_id.personal.get(&PersonalIdTag::GivenName).unwrap_or(&empty_item).value;
    let birthdate = format_date(&personal_id.personal.get(&PersonalIdTag::DateOfBirth).unwrap_or(&empty_item).value);
    
    let place_of_birth = &personal_id.personal.get(&PersonalIdTag::PlaceOfBirth).unwrap_or(&empty_item).value;
    let state_of_birth = &personal_id.personal.get(&PersonalIdTag::StateOfBirth).unwrap_or(&empty_item).value;
    let community_of_birth = &personal_id.personal.get(&PersonalIdTag::CommunityOfBirth).unwrap_or(&empty_item).value;
    let parent = &personal_id.personal.get(&PersonalIdTag::ParentGivenName).unwrap_or(&empty_item).value;
    let state = &personal_id.personal.get(&PersonalIdTag::State).unwrap_or(&empty_item).value;
    let community = &personal_id.personal.get(&PersonalIdTag::Community).unwrap_or(&empty_item).value;
    let address = &personal_id.personal.get(&PersonalIdTag::Street).unwrap_or(&empty_item).value;


    let place = &personal_id.personal.get(&PersonalIdTag::Place).unwrap_or(&empty_item).value;
    let house_number = &personal_id.personal.get(&PersonalIdTag::HouseNumber).unwrap_or(&empty_item).value;
    let house_letter = &personal_id.personal.get(&PersonalIdTag::HouseLetter).unwrap_or(&empty_item).value;
    let entrance = &personal_id.personal.get(&PersonalIdTag::Entrance).unwrap_or(&empty_item).value;
    let floor = &personal_id.personal.get(&PersonalIdTag::Floor).unwrap_or(&empty_item).value;
    let appartment_number = &personal_id.personal.get(&PersonalIdTag::AppartmentNumber).unwrap_or(&empty_item).value;

    let mut address_date = format_date(&personal_id.personal.get(&PersonalIdTag::AddressDate).unwrap_or(&empty_item).value); 
    if address_date == "01.01.0001." {
        address_date = "Nije dostupan".to_string();
    }

    let personal_number = &personal_id.personal.get(&PersonalIdTag::PersonalNumber).unwrap_or(&empty_item).value;
    let gender = &personal_id.personal.get(&PersonalIdTag::Sex).unwrap_or(&empty_item).value;
    let authority = &personal_id.personal.get(&PersonalIdTag::IssuingAuthority).unwrap_or(&empty_item).value;
    let id_no = &personal_id.personal.get(&PersonalIdTag::DocRegNo).unwrap_or(&empty_item).value;
    let issuing_date = format_date(&personal_id.personal.get(&PersonalIdTag::IssuingDate).unwrap_or(&empty_item).value);
    let expiry_date = format_date(&personal_id.personal.get(&PersonalIdTag::ExpiryDate).unwrap_or(&empty_item).value);

    let date_now = chrono::offset::Local::now();
    let print_date = format!("{}", date_now.format("%d.%m.%Y."));

    add_line(left_margin, 277.0, &current_layer);
    add_text(left_margin+2.0, 269.0, "ČITAČ ELEKTRONSKE LIČNE KARTE: ŠTAMPA PODATAKA", 15.5, &font2, &current_layer);
    add_line(left_margin, 265.0, &current_layer);

    add_line(left_margin, 196.0, &current_layer);
    add_text(left_margin+2.0, 190.0, "Podaci o građaninu", 12.0, &font2, &current_layer);
    add_line(left_margin, 187.0, &current_layer);

    add_text(left_margin+2.0, 181.0, "Prezime:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 181.0, surname, 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 173.0, "Ime:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 173.0, name, 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 165.0, "Ime jednog roditelja:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 165.0, parent, 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 157.0, "Datum rođenja:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 157.0, &birthdate.as_str(), 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 148.0, "Mesto rođenja, opština i", 11.0, &font2, &current_layer);
    add_text(left_margin+2.0, 144.0, "država:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 146.0, &[place_of_birth,", ",community_of_birth,", ",state_of_birth].to_vec().concat(), 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 135.0, "Prebivalište i adresa", 11.0, &font2, &current_layer);
    add_text(left_margin+2.0, 131.0, "stana:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 133.0, &[place, ", ", community, ", ", address, " ", house_number,house_letter,"/",entrance,"/",floor,"/",appartment_number].to_vec().concat(), 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 121.0, "Datum promene adrese:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 121.0, &address_date.as_str(), 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 111.0, "JMBG:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 111.0, personal_number, 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 101.0, "Pol:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 101.0, gender, 11.0, &font2, &current_layer);

    add_line(left_margin, 97.0, &current_layer);
    add_text(left_margin+2.0, 91.5, "Podaci o dokumentu", 12.0, &font2, &current_layer);
    add_line(left_margin, 88.0, &current_layer);

    add_text(left_margin+2.0, 81.0, "Dokument izdaje:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 81.0, authority, 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 73.0, "Broj dokumenta:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 73.0, id_no, 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 65.0, "Datum izdavanja:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 65.0, &issuing_date.as_str(), 11.0, &font2, &current_layer);

    add_text(left_margin+2.0, 57.0, "Važi do:", 11.0, &font2, &current_layer);
    add_text(left_margin+data_margin, 57.0, &expiry_date.as_str(), 11.0, &font2, &current_layer);

    add_line(left_margin, 54.0, &current_layer);
    add_line(left_margin, 53.0, &current_layer);

    add_text(left_margin+2.0, 45.0, &["Datum štampe: ", &print_date.as_str()].to_vec().concat(), 11.0, &font2, &current_layer);


    let disclamer1 = "1. U čipu lične karte, podaci o imenu i prezimenu imaoca lične karte ispisani su na nacionalnom pismu onako kako su";
    let disclamer2 = "ispisani na samom obrascu lične karte, dok su ostali podaci ispisani latiničkim pismom.";
    let disclamer3 = "2. Ako se ime lica sastoji od dve reči čija je ukupna dužina između 20 i 30 karaktera ili prezimena od dve reči čija je";
    let disclamer4 = "ukupna dužina između 30 i 36 karaktera, u čipu lične karte izdate pre 18.08.2014. godine, druga reč u imenu ili prezimenu";
    let disclamer5 = "skraćuje se na prva dva karaktera.";


    add_line(left_margin, 35.0, &current_layer);
    add_text(left_margin, 31.0, &disclamer1, 8.8, &font2, &current_layer);
    add_text(left_margin, 28.0, &disclamer2, 8.8, &font2, &current_layer);
    add_text(left_margin, 25.0, &disclamer3, 8.8, &font2, &current_layer);
    add_text(left_margin, 22.0, &disclamer4, 8.8, &font2, &current_layer);
    add_text(left_margin, 19.0, &disclamer5, 8.8, &font2, &current_layer);    
    add_line(left_margin, 15.0, &current_layer);



    add_image(left_margin, 203.0, &personal_id.image, &current_layer).unwrap();
    let pdf_file = match File::create(&[path,"/", "Podaci iz lične karte osobe ", name, " ", surname, "-",  &date_now.format("%Y").to_string(), ".pdf"].concat()) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };

    match doc.save(&mut BufWriter::new(pdf_file)) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}
