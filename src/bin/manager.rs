use native_dialog::{FileDialog, MessageDialog};
use slint::Model;
use slint::{SharedString, StandardListViewItem};

use student_record;
use database;
use error_report;
use error_report::VisualErrorReport;
use nhs_hours_tracker::*;

const PERSONAL_FORMAT: fn(bool) -> String = |i| if i { "Personal" } else { "Group" }.into();
const SEMESTER_FORMAT: fn(bool) -> String = |i| if i { "S2" } else { "S1" }.into();
const USER_FORMAT: fn((String, String, String, u32, u32)) -> String =
    |(f, m, l, c, i)| format!("{} {} {} -- Class: {} Inducted: {}", f, m, l, c, i);
const SERVICE_FORMAT: fn((String, String, String, u32, u32, u32, bool, bool, u32)) -> String =
    |(f, m, l, c, i, y, sem, per, hours)| {
        format!(
            "{} -- {} {} Hours Semester {} {} ",
            USER_FORMAT((f, m, l, c, i)),
            hours,
            PERSONAL_FORMAT(per),
            SEMESTER_FORMAT(sem),
            y
        )
    };




fn main() {
    let db = std::rc::Rc::from(database::NhsDb::new());
    let main = std::rc::Rc::from(MainWindow::new());

    {
        let db = db.clone();
        main.global::<Search>().on_search_student(move |i,_| {
            let mut i = i.iter();
            let searched = db.search_people(
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
            );
            let output: Vec<StandardListViewItem> = searched
                .into_iter()
                .map(USER_FORMAT)
                .map(|x| x.into())
                .map(|x: slint::SharedString| x.into())
                .collect();
            std::rc::Rc::new(slint::VecModel::from(output)).into()
        });
    }
    {
        let db = db.clone();
        main.global::<Search>().on_search_student_record(move |i,_| {

            let mut i = i.iter();
            let searched = db.search_service(
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
                i.next().unwrap().into(),
            );
            let output: Vec<StandardListViewItem> = searched
                .into_iter()
                .map(SERVICE_FORMAT)
                .map(|x| x.into())
                .map(|x: slint::SharedString| x.into())
                .collect();
            std::rc::Rc::new(slint::VecModel::from(output)).into()
        });
    }
    {
        let db = db.clone();
        main.global::<Search>().on_get_students(move || {
            let o: Vec<StandardListViewItem> = db
                .get_users()
                .into_iter()
                .map(USER_FORMAT)
                .map(SharedString::from)
                .map(StandardListViewItem::from)
                .collect();
            std::rc::Rc::new(slint::VecModel::from(o)).into()
        });
    }
    {
        let db = db.clone();
        main.global::<Interface>().on_create_person(move |i| {
            let mut i = i.iter();
            let f = i.next().unwrap().to_string();
            let m = i.next().unwrap().to_string();
            let l = i.next().unwrap().to_string();
            let c = err!(i.next().unwrap().to_string().parse().unwrap_rep_msg("Class Year feild must be filled out correctly to create a person!"));
            let ind = err!(i.next().unwrap().to_string().parse().unwrap_rep_msg("Induction Year feild must be filled out correctly to create a person!"));
            let id = err!(i.next().unwrap().to_string().parse().unwrap_rep_msg("School Id must be filled out correctly to create a person"));

            let msg = format!("Is the following correct?\n Name : {} {} {}\n Class: {} Inducted: {} School ID: {}",f,m,l,c,ind,id);
            let ok = err!(MessageDialog::new().set_title("Is This Correct?").set_text(&msg).show_confirm().unwrap_rep_msg("Failed to Create 'Is This Correct Alert'"));

            if ok {
                db.new_person(f, m, l, c, ind,id);
            }
        });
    }
    {
        let db = db.clone();
        main.global::<Interface>()
            .on_create_service_logs(move |date, s1, s2, g, p, search| {
                let mut i = search.iter();
                let f = i.next().unwrap().to_string();
                let m = i.next().unwrap().to_string();
                let l = i.next().unwrap().to_string();
                let c = i.next().unwrap().to_string().parse().unwrap_or_default();
                let i = i.next().unwrap().to_string().parse().unwrap_or_default();
                db.create_service_logs(err!(date.try_into().unwrap_rep_msg("Date feild must be filled out correctly to create service logs")), s1, s2, g, p, f, m, l, c, i);
            });
    }
    {
        let db = db.clone();
        main.global::<Interface>()
            .on_add_service_hours(move |add, search| {
                let mut it = search.iter();
                let f = it.next().unwrap().to_string();
                let m = it.next().unwrap().to_string();
                let l = it.next().unwrap().to_string();
                let c = it.next().unwrap().to_string();
                let i = it.next().unwrap().to_string();
                let y = it.next().unwrap().to_string();
                let s1 = it.next().unwrap().to_string();
                let p = it.next().unwrap().to_string();
                db.add_service_hours(err!(add.try_into().unwrap_rep_msg("Add feild must be filled out correctly to add to service logs")), f, m, l, c, i, y, s1, p);
            });
    }
    {
        let db = db.clone();
        main.global::<Interface>()
            .on_sum_service_hours(move |search| {
                let mut it = search.iter();
                let f = it.next().unwrap().to_string();
                let m = it.next().unwrap().to_string();
                let l = it.next().unwrap().to_string();
                let c = it.next().unwrap().to_string();
                let i = it.next().unwrap().to_string();
                let y = it.next().unwrap().to_string();
                let s1 = it.next().unwrap().to_string();
                let p = it.next().unwrap().to_string();
                db.sum_service_hours(f, m, l, c, i, y, s1, p)
                    .try_into()
                    .unwrap_rep_msg("Failed to convert total service hours to correct type\n Wow you propbably overloaded it").unwrap_or_default()
            });

    }
    {
        let db = db.clone();
        let main1 = main.clone();
        main.global::<Interface>()
            .on_prepare_induction_canidates(move || {
                let diolog = FileDialog::new().add_filter("CSV Files", &["csv"]);
                let file = diolog.show_open_single_file();
                if let Ok(Some(file)) = file{

                    let ids = db.get_student_ids();
                    let mut file = err!(csv::Reader::from_path(file).unwrap_rep_msg("Failed to Open CSV File"));

                    //dbg!(&ids);

                    let list: Vec<InductionCanidate> = file.deserialize().take_while(|r| r.is_ok()).flat_map(|r| r.ok()).filter(|record: &student_record::Record| ids.binary_search(&record.ALTSTUDENTID).is_err()).map(|record: student_record::Record| record.into()).collect();

                    if !file.is_done(){
                        let diolog = MessageDialog::new().set_title("Failed to Parse File").set_text("Failed to parse selected file");
                        diolog.show_alert().expect("Failed to create Error Message Box");
                        return;
                    }

                    main1.global::<Interface>().set_canidates(slint::ModelRc::new(slint::VecModel::from(list)));

                } else {
                    let diolog = MessageDialog::new().set_title("Failed to Select File").set_text("Failed to select induction file");
                    diolog.show_alert().expect("Failed to create Error Message Box");
                    return;
                }

            });
    }
    {
        let main1 = main.clone();
        main.global::<Interface>()
            .on_sort_induction_canidates(move || {
                let canidates = main1.global::<Interface>().get_canidates();
                let mut canidates: Vec<InductionCanidate> = canidates.iter().collect();
                canidates.sort_by_key(|canidate| -1 * canidate.catigories.iter().fold(0,|sum,catagory| sum + catagory));
                //dbg!(canidates);
                main1.global::<Interface>().set_canidates(slint::ModelRc::new(slint::VecModel::from(canidates)));

            });
    }
    {
        let main1 = main.clone();
        let db = db.clone();
        main.global::<Interface>()
            .on_induct_canidates(move || {
                let canidates = main1.global::<Interface>().get_canidates();
                let canidates: Vec<InductionCanidate> = canidates.iter().collect();

                db.induct_members(canidates);


            });
    }
    {
        let db = db.clone();
        main.global::<Interface>()
            .on_get_libo_induction_canidates(move |_| {
                slint::ModelRc::new(slint::VecModel::from(db.get_libo_canidates()))
            });
    }
    {
        let db = db.clone();
        main.global::<Interface>()
            .on_make_decision_on_libo_canidate(move |id,dec| {
                db.induct_member(id,dec);
            });
    }
    {
        let db = db.clone();
        main.global::<Interface>()
            .on_export_induction_logs(move || {
                db.export_induction_logs();
            });
    }

    main.run();
}
