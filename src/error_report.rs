use core::fmt::Display;
use native_dialog::MessageDialog;


#[macro_export]
macro_rules! err {
    ( $x:expr ) => {
        {
            if let Some(a) = $x{
                a
            } else {
                return
            }
        }
    };
}

pub trait VisualErrorReport<T> {
    fn unwrap_rep(self) -> Option<T>;
    fn unwrap_rep_msg<M: Display>(self,m: M) -> Option<T>;
}

impl<T,E: Display> VisualErrorReport<T> for Result<T,E>{
    fn unwrap_rep(self) -> Option<T>{
        match self{
            Ok(a) => Some(a),
            Err(a) => {
                let msg = format!("{}",a);
                let dio = MessageDialog::new().set_title("An Error Occured").set_text(&msg);
                dio.show_alert().expect(&format!("Failed to Show Error Diolog with {}",msg));
                None
            }
        }
    }
    fn unwrap_rep_msg<M: Display>(self,m: M) -> Option<T>{
        match self{
            Ok(a) => Some(a),
            Err(a) => {
                let msg = format!("{}\n'{}'",m,a);
                let dio = MessageDialog::new().set_title("An Error Occured").set_text(&msg);
                dio.show_alert().expect(&format!("Failed to Show Error Diolog with {}",msg));
                None
            }
        }
    }
}