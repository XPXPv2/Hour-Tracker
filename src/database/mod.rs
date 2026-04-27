pub use rusqlite::{params, Connection, Transaction};
use std::path::Path;
mod new;

use super::error_report::VisualErrorReport;
use super::err;
use std::cell::RefCell;

static DATA_BASE_FILE_NAME: &str = "nhs_membership.db";
const NOT_RETURNED_STRING: &str = "Form not returned";
const INDUCTED_STRING: &str = "Induct";
pub const NOT_DECIED_STRING: &str = "Not Reviewed";

fn string_to_bool(i: String) -> bool {
    match i.as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("Unreconized boolean string '{}'", i),
    }
}

pub struct NhsDb {
    con: RefCell<Connection>,
}

impl NhsDb {
    pub fn new() -> Self {
        let con = if Path::new(DATA_BASE_FILE_NAME).exists() {
            Connection::open(DATA_BASE_FILE_NAME).unwrap()
        } else {
            new::new_db(DATA_BASE_FILE_NAME)
        };
        
        let con = RefCell::new(con);

        Self { con }
    }
    pub fn new_person(&self, f: String, m: String, l: String, class: u32, induct: u32, school_id:u32) {
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
    
        con
            .execute(
                "INSERT INTO people (id,first,mi,last,class,inducted) VALUES (?6,?1,?2,?3,?4,?5);",
                params![f, m, l, class, induct,school_id],
            )
            .unwrap();
         
         con.commit().unwrap();
    }
    pub fn get_users(&self) -> Vec<(String, String, String, u32, u32)> {
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
    
        let mut state = 
            con
            .prepare("SELECT first, mi,last,class,inducted from people order by last;")
            .unwrap();
        let r: Vec<(String, String, String, u32, u32)> = state
            .query_map([], |row| {
                Ok((row.get(0), row.get(1), row.get(2), row.get(3), row.get(4)))
            })
            .unwrap()
            .map(|x| x.unwrap())
            .map(|(x, y, z, a, b)| (x.unwrap(), y.unwrap(), z.unwrap(), a.unwrap(), b.unwrap()))
            .collect();
            
        drop(state);
            
        con.commit().unwrap();
        r
    }
    pub fn get_student_ids(&self) -> Vec<usize>{
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
    
    
        let mut state = 
            con
            .prepare("SELECT id from people order by id;")
            .unwrap();
        let r: Vec<usize> = state
            .query_map([], |row| {
                Ok(row.get(0))
            })
            .unwrap()
            .map(|x| x.unwrap())
            .map(|x| x.unwrap())
            .collect();
         
         
         drop(state);
         con.commit().unwrap();
        r
    }
    pub fn search_people(
        &self,
        f: String,
        m: String,
        l: String,
        c: String,
        i: String,
    ) -> Vec<(String, String, String, u32, u32)> {
    
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
    
        let mut state = con.prepare("SELECT first, mi,last,class,inducted from people where first like ?1 and mi like ?2 and last like ?3 and class like ?4 and inducted like ?5;").unwrap();
        let r: Vec<(String, String, String, u32, u32)> = state
            .query_map([f + "%", m + "%", l + "%", c + "%", i + "%"], |row| {
                Ok((row.get(0), row.get(1), row.get(2), row.get(3), row.get(4)))
            })
            .unwrap()
            .map(|x| x.unwrap())
            .map(|(x, y, z, a, b)| (x.unwrap(), y.unwrap(), z.unwrap(), a.unwrap(), b.unwrap()))
            .collect();
            
         drop(state);
         con.commit().unwrap();
        r
    }
    
    pub fn create_service_logs(
        &self,
        date: u32,
        s1: bool,
        s2: bool,
        g: bool,
        p: bool,
        f: String,
        m: String,
        l: String,
        c: String,
        i: String,
    ) {
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
           Self::create_service_logs_inner(&con,date,s1,s2,g,p,f,m,l,c,i);
        con.commit().unwrap();
    }
    

    fn create_service_logs_inner(
        con: &Transaction<'_>,
        date: u32,
        s1: bool,
        s2: bool,
        g: bool,
        p: bool,
        f: String,
        m: String,
        l: String,
        c: String,
        i: String,
    ) {
    
        if !(s1 || s2) || !(g || p) {
            return;
        }

        if s1 && s2 {
            Self::create_service_logs_inner(
                con,
                date,
                true,
                false,
                g,
                p,
                f.clone(),
                m.clone(),
                l.clone(),
                c.clone(),
                i.clone(),
            );
            Self::create_service_logs_inner(
                con,
                date,
                false,
                true,
                g,
                p,
                f.clone(),
                m.clone(),
                l.clone(),
                c.clone(),
                i.clone(),
            );
            return;
        }
        if g && p {
            Self::create_service_logs_inner(
                con,
                date,
                s1,
                s2,
                true,
                false,
                f.clone(),
                m.clone(),
                l.clone(),
                c.clone(),
                i.clone(),
            );
            Self::create_service_logs_inner(
                con,
                date,
                s1,
                s2,
                false,
                true,
                f.clone(),
                m.clone(),
                l.clone(),
                c.clone(),
                i.clone(),
            );
            return;
        }
        //semeser == false == s1;
        let semester = !s1;
        let personal = p;
        con.execute("INSERT INTO service (id, year, semester,personal,hours) SELECT id , ?6, ?7, ?8, 0 FROM people WHERE first like ?1 and mi like ?2 and last like ?3 and class like ?4 and inducted like ?5;",[f + "%",m + "%",l + "%",c + "%", i + "%",date.to_string(),semester.to_string(),personal.to_string()]).unwrap();
    }

    pub fn search_service(
        &self,
        f: String,
        m: String,
        l: String,
        c: String,
        i: String,
        year: String,
        sem: String,
        per: String,
    ) -> Vec<(String, String, String, u32, u32, u32, bool, bool, u32)> {
    
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
    
        let sem = match sem.as_str() {
            "0" => "",
            "1" => "false",
            "2" => "true",
            "" => "",
            _ => panic!("invalid {}", sem),
        }
        .to_string();

        let per = match per.as_str() {
            "0" => "",
            "1" => "false",
            "2" => "true",
            "" => "",
            _ => panic!("invalid {}", sem),
        }
        .to_string();

        let mut state = con.prepare("SELECT first, mi,last,class,inducted,year,semester,personal,hours from people inner join service on service.id == people.id and people.first like ?1 and people.mi like ?2 and people.last like ?3 and people.class like ?4 and people.inducted like ?5 and service.year like ?6 and service.semester like ?7 and service.personal like ?8 ;").unwrap();
        let r: Vec<(String, String, String, u32, u32, u32, bool, bool, u32)> = state
            .query_map(
                [
                    f + "%",
                    m + "%",
                    l + "%",
                    c + "%",
                    i + "%",
                    year + "%",
                    sem + "%",
                    per + "%",
                ],
                |row| {
                    Ok((
                        row.get(0),
                        row.get(1),
                        row.get(2),
                        row.get(3),
                        row.get(4),
                        row.get(5),
                        row.get(6),
                        row.get(7),
                        row.get(8),
                    ))
                },
            )
            .unwrap()
            .map(|x| x.unwrap())
            .map(|(x, y, z, a, b, c, d, e, f)| {
                (
                    x.unwrap(),
                    y.unwrap(),
                    z.unwrap(),
                    a.unwrap(),
                    b.unwrap(),
                    c.unwrap(),
                    string_to_bool(d.unwrap()),
                    string_to_bool(e.unwrap()),
                    f.unwrap(),
                )
            })
            .collect();
            
            drop(state);
            
            
            con.commit().unwrap();
        r
    }

    pub fn add_service_hours(
        &self,
        to_add: i32,
        f: String,
        m: String,
        l: String,
        c: String,
        i: String,
        year: String,
        sem: String,
        per: String,
    ) {
    
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
        let sem = match sem.as_str() {
            "0" => "",
            "1" => "false",
            "2" => "true",
            "" => "",
            _ => panic!("invalid {}", sem),
        }
        .to_string();

        let per = match per.as_str() {
            "0" => "",
            "1" => "true",
            "2" => "false",
            "" => "",
            _ => panic!("invalid {}", sem),
        }
        .to_string();

        con.execute(
            "update service set hours = MAX(service.hours + ?9,0) where exists(select * from people where service.id == people.id and people.first like ?1 and people.mi like ?2 and people.last like ?3 and people.class like ?4 and people.inducted like ?5 and service.year like ?6 and service.semester like ?7 and service.personal like ?8) ;",
            params![f + "%",m + "%",l + "%",c + "%", i + "%",year+"%", sem + "%", per + "%",to_add]
        ).unwrap();
        
        con.commit().unwrap();
    }
    pub fn sum_service_hours(
        &self,
        f: String,
        m: String,
        l: String,
        c: String,
        i: String,
        year: String,
        sem: String,
        per: String,
    ) -> u32 {
    
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
        let sem = match sem.as_str() {
            "0" => "",
            "1" => "false",
            "2" => "true",
            "" => "",
            _ => panic!("invalid {}", sem),
        }
        .to_string();

        let per = match per.as_str() {
            "0" => "",
            "1" => "true",
            "2" => "false",
            "" => "",
            _ => panic!("invalid {}", sem),
        }
        .to_string();

        let r = if let Some(r) = con.query_row(
            "SELECT sum(hours) from people inner join service on service.id == people.id and people.first like ?1 and people.mi like ?2 and people.last like ?3 and people.class like ?4 and people.inducted like ?5 and service.year like ?6 and service.semester like ?7 and service.personal like ?8 ;",
            params![f + "%",m + "%",l + "%",c + "%", i + "%",year+"%", sem + "%", per + "%"],
            |row| row.get(0)
        ).unwrap_rep(){
            r
        } else {
            return 0
        };
        
        con.commit().unwrap();
        r
    }

    pub fn induct_members(&self,canidates: Vec<super::InductionCanidate>){
        use chrono::Datelike;
        
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();

        let mut sql_ind_cmd = con.prepare_cached("INSERT INTO inductions (first,last,year,grade,returned,inducted,ceramony,school_id,mi,notes) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10);").unwrap();
        //let mut sql_per_cmd = con.prepare_cached("INSERT INTO people (id,first,mi,last,class,inducted) VALUES (?1,?2,?3,?4,?5,?6);").unwrap();

        let year = chrono::Utc::today().naive_local().year();

        let msg: String = canidates.iter().filter(|canidate| canidate.decision == NOT_DECIED_STRING).map(|canidate| {
            format!("{} {} decision has not been made",canidate.first_name,canidate.last_name)
        }).collect::<Vec<String>>().join("\n");

        if !msg.trim().is_empty(){
            use native_dialog::MessageDialog;
            MessageDialog::new().set_title("Undecided Student").set_text(&msg).show_alert().expect("Failed to show alert");
            return;
        }

        canidates.iter().for_each(|canidate|{
            let returned = canidate.decision != NOT_RETURNED_STRING;
            let inducted = canidate.decision == INDUCTED_STRING;
            let ceramony: Option<bool> = if inducted {None} else {Some(false)} ;
            let mi = if inducted {Some(canidate.middle_inital.to_string())} else {None};
            let school_id = if inducted {Some(canidate.id)} else {None};
            let notes = if inducted {"".to_string()} else {canidate.decision.to_string()};
            sql_ind_cmd.execute(params![canidate.first_name.to_string(),canidate.last_name.to_string(),year,canidate.grade,returned,inducted,ceramony,school_id,mi,notes]).unwrap();
        });

        //canidates.sort_by_key(|canidate| canidate.id);
        //canidates.dedup_by_key(|canidate| canidate.id);
        drop(sql_ind_cmd);
        //canidates.iter().filter(|canidate| canidate.decision == INDUCTED_STRING).for_each(|canidate|{
        //    let class = 12 - canidate.grade + 1 + year;
        //    sql_per_cmd.execute(params![canidate.id,canidate.first_name.to_string(),canidate.middle_inital.to_string(),canidate.last_name.to_string(), class, year]).unwrap_rep_msg("Failed to ");
        //});
        
        con.commit().unwrap();

    }

    pub fn get_libo_canidates(&self) -> Vec<super::LiboCanidateInfo>{
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
    
        let mut state = con.prepare("SELECT id,first,last,ceramony FROM inductions WHERE ceramony IS NULL;").unwrap();
    
        let r = state.query_map(
            [],
            |row| {
                Ok((
                    row.get(0),
                    row.get(1),
                    row.get(2),
                ))
            },
        )
        .unwrap()
        .map(|x| x.unwrap())
        .map(|(x, y, z): (Result<i32,_>,Result<String,_>,Result<String,_>)| {
            super::LiboCanidateInfo{
                id: x.unwrap(),
                first_name: slint::SharedString::from(y.unwrap()),
                last_name: slint::SharedString::from(z.unwrap()),
            }
        })
        .collect();
        drop(state);
        con.commit().unwrap();
        r
    }

    //pub fn new_person(&self, f: String, m: String, l: String, class: u32, induct: u32, school_id:u32) {

    pub fn induct_member(&self,id:i32,decision:bool){
        let mut conref = self.con.borrow_mut();let con = conref.transaction().unwrap();
        if decision{
            let (id,first,mi,last,grade,year):(u32, std::string::String, std::string::String, std::string::String, u32, u32) = con
                .query_row(
                    "SELECT school_id,first,mi,last,grade,year FROM inductions WHERE id=?1;",
                    params![id],
                    |row|{
                        Ok((row.get(0).unwrap(),row.get(1).unwrap(),row.get(2).unwrap(),row.get(3).unwrap(),row.get(4).unwrap(),row.get(5).unwrap()))
                    }
                )
                .unwrap();

            let class:u32 = 12 - grade + 1 + year;

            self.new_person(first, mi, last, class, year, id);
            
        }

        con.execute(
                "UPDATE inductions SET ceramony=?1 WHERE id=?2;",
                params![Some(decision),id],
            ).unwrap();
        con.commit().unwrap();
    }

    pub fn export_induction_logs(&self){
        use docx_rs::*;

        const BOOL_FMT: fn(bool) -> String = |i| if i {"x"} else {" "}.into();

        let path = err!(err!(native_dialog::FileDialog::new().add_filter("Text Document",&["docx"]).show_save_single_file().unwrap_rep()));
        let file = std::fs::File::create(&path).unwrap();

        let header = ["Candidate Insert names, alpha order (Last name, First name)","Grade","Candidate Form Sent","Not Interested in Membership/Candidate Form Not Returned","SELECTED","Not Selected","Induction Ceremony Invitation Sent","Attended Induction Ceremony","Reason for Non Selection (identify criteria not met)"];
        let header = TableRow::new(header.into_iter().map(|string| TableCell::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(string).size(12)))).collect());

        let mut table = vec![header];
        let conref = self.con.borrow();
        let mut state = conref.prepare("SELECT first, last, year,grade,returned,inducted,ceramony,notes FROM inductions WHERE ceramony IS NOT NULL ORDER BY last, first;").unwrap();
    
        let mut contents = state.query_map(
            [],
            |row| {
                Ok((
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                ))
            },
        )
        .unwrap()
        .map(|x| x.unwrap())
        .map(|(f,l,y,g,r,i,c,n): (String,String,i32,i32,bool,bool,bool,String)| {
            let names = f + " " + &l + "/" + &y.to_string();
            let grade = g.to_string();
            let form = BOOL_FMT(true);
            let nret = BOOL_FMT(!r);
            let selected = BOOL_FMT(i);
            let not_selected = BOOL_FMT(!i & r);
            let invite = BOOL_FMT(i);
            let attended = BOOL_FMT(c);
            let notes = if i & !c {"Did not attend inducion ceramony".to_string()} else {n};

            [names,grade,form,nret,selected,not_selected,invite,attended,notes]

        })
        .map(|cols|{
            let row = cols.into_iter().map(|cell| TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(cell).size(22)))).collect();
            TableRow::new(row)
        })
        .collect();
        
        table.append(&mut contents);


        let bsize = 8;
        let boarders = TableBorders::new();
        let boarders = boarders.set(TableBorder::new(TableBorderPosition::Top).size(bsize));
        let boarders = boarders.set(TableBorder::new(TableBorderPosition::Left).size(bsize));
        let boarders = boarders.set(TableBorder::new(TableBorderPosition::Bottom).size(bsize));
        let boarders = boarders.set(TableBorder::new(TableBorderPosition::Right).size(bsize));
        let boarders = boarders.set(TableBorder::new(TableBorderPosition::InsideH).size(bsize));
        let boarders = boarders.set(TableBorder::new(TableBorderPosition::InsideV).size(bsize));


        let table = Table::new(table)
        .layout(TableLayoutType::Autofit)
        .set_borders(boarders)
        .align(TableAlignmentType::Center);


        //let _header_pic = Pic::new(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/nhs_logo.png")));


        let header = Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Unfortchenletly the library used to create this file is broken when inserting images in headers and so this has to be done manualy for now :'(")));
        let footer = Footer::new().add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().add_text("National Association of Secondary School Principals • 1904 Association Drive, Reston VA 20191").add_break(BreakType::TextWrapping)).add_run(Run::new().add_text("nhs.us • njhs.us").bold()));


        Docx::new().add_table(table).header(header).footer(footer).page_margin(PageMargin::new().left(0).right(0)).build().pack(file).unwrap();

    }
}
