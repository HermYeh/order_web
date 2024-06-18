use chrono::{DateTime, FixedOffset, Local};

use egui::{Align, Color32, Label, Stroke, TextStyle};
use chrono::TimeZone;
use ehttp::Request;
use std::io::Bytes;



use super::app;
use app::TemplateApp;

use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug,PartialEq,Eq)]
pub struct Order {
    pub order_number:  Box<String>,
    pub check_in:  Box<String>,
    pub payment:  Box<String>,
}
impl fmt::Display for Order {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self)
    }
}
impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_number.cmp(&other.order_number)
    }
}

pub struct Table {
    striped: bool,
   
 
   
}

use std::ffi::CString;
use std::mem::size_of;
use std::slice;
use std::{
    io::{prelude::*, BufReader},
    str,
};
use std::net::TcpStream;
struct FileHeader {
    size: u32,
    
}
impl Default for Table {
    fn default() -> Self {
        Self {
           
            striped: true,
          
        
            
        }
    }
}
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Msg {
    vector:Vec<(String,String,String)>,
}

use serde_json::value::Serializer;
use serde_json::Deserializer;
pub fn save_to_remote(total_order:Order)  {
 
    
    
     
  
    
  
   
     let request = Request{
        headers: ehttp::Headers::new(&[
            ("Content-Type", "application/json"),
        ]),..Request::json("https://settingupdate.com/new/order.php",&total_order).unwrap()};
  
    
        ehttp::fetch(request, move |response| {
 
          //  println!("Response: {:?}",response);
            
         });
     


}
pub fn update_to_remote(order:Order)  {
 
    
    
     
  
    
  
   
    let request = Request{
       headers: ehttp::Headers::new(&[
           ("Content-Type", "application/json"),
       ]),..Request::json("https://settingupdate.com/new/delete.php",&order).unwrap()};
 
   
       ehttp::fetch(request, move |response| {

        //   println!("Response: {:?}",response);
           
        });
    


}

impl Table {
    
   pub fn table_ui(&mut self, ui: &mut egui::Ui,table_data:&mut TemplateApp) {
        use egui_extras::{Column, TableBuilder};
    
   
    let s = Stroke {
            width: 0.0,
            color: Color32::TRANSPARENT,
        };
    let available_height = ui.available_height()-table_data.height-130.0;
    let wsize=ui.available_width();
    let mut table = TableBuilder::new(ui).cell_layout(egui::Layout::left_to_right(egui::Align::TOP))
        .column(Column::exact(wsize/4.00-10.0))
        .column(Column::exact(wsize/4.00-10.0))
        .column(Column::exact(wsize/4.00-10.0))
        .column(Column::exact(wsize/4.00-10.0))
        .resizable(false)
        .striped(self.striped)
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);
        
        table = table.sense(egui::Sense::click());
        if let Some(row_index) = table_data.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, Some(Align::TOP));
        }
    
                
        
    table.header(20.0, |mut header| {
        header.col(|ui| {
         
        let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Order#").fill(egui::Color32::TRANSPARENT));
        if sort_click.clicked(){
            let data_temp= table_data.total_order.lock().unwrap().clone();
            table_data.total_order.lock().unwrap().sort();
            if data_temp== *table_data.total_order.lock().unwrap(){
                table_data.total_order.lock().unwrap().reverse();
            }
          
        }
        
        });
        header.col(|ui| {
            let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Check In").fill(egui::Color32::TRANSPARENT));
            if sort_click.clicked(){
                let data_temp= table_data.total_order.lock().unwrap().clone();
                table_data.total_order.lock().unwrap().sort_by_key(|k| DateTime::parse_from_rfc3339(&k.check_in).unwrap());
                if data_temp== *table_data.total_order.lock().unwrap(){
                    table_data.total_order.lock().unwrap().reverse();
                }
              
            }
         

        });
        header.col(|ui| {
            let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Wait Time").fill(egui::Color32::TRANSPARENT));
            if sort_click.clicked(){
                let data_temp= table_data.total_order.lock().unwrap().clone();
                table_data.total_order.lock().unwrap().sort_by_key(|k| DateTime::parse_from_rfc3339(&k.check_in).unwrap());
                if data_temp== *table_data.total_order.lock().unwrap(){
                    table_data.total_order.lock().unwrap().reverse();
                }
              
            }
        });
        header.col(|ui| {
          ui.add_sized(ui.available_size(),egui::Button::new("Notify").fill(egui::Color32::TRANSPARENT));
     
        })
        ;
        })
        
    .body(|mut body| {
    let order_size=table_data.total_order.lock().unwrap().len();
    for row_index in 0..order_size {    
        if row_index>=table_data.total_order.lock().unwrap().len(){
            continue;
         }
        body.row(20.0, |mut row| {
            let rowindex=row.index();
            if table_data.selection==row_index{
                row.set_selected(true);
                
            }
            
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(*table_data.total_order.lock().unwrap()[rowindex].order_number.clone()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(DateTime::parse_from_rfc3339(&table_data.total_order.lock().unwrap()[rowindex].check_in.clone()).unwrap().format("%H:%M").to_string()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let time_now: DateTime<Local> = Local::now();
                let time_wait = time_now.to_utc()-(DateTime::parse_from_rfc3339(&table_data.total_order.lock().unwrap()[rowindex].check_in.clone()).unwrap().to_utc());
        
             
                let minutes = (time_wait.num_minutes()).to_string();
               
               
               let time= minutes+" min";
                ui.add_sized( ui.available_size(),Label::new(egui::RichText::new(time).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let response = ui
                .add_sized(
                    ui.available_size(),
                    egui::Button::new(if *table_data.total_order.lock().unwrap()[rowindex].payment=="1" {"Sent"}else if *table_data.total_order.lock().unwrap()[rowindex].payment=="2" {"Received"} else {""}),
                );
                if response.clicked(){
                 if *table_data.total_order.lock().unwrap()[rowindex].payment=="0"{
                    
                    app::TemplateApp::send_notification(*table_data.total_order.lock().unwrap()[rowindex].order_number.clone());
                    table_data.logs.push(format!("Sent order #{} notification to kitchen!",*table_data.total_order.lock().unwrap()[rowindex].order_number.clone()));
                    *table_data.total_order.lock().unwrap()[rowindex].payment="1".to_string();
                    update_to_remote(table_data.total_order.lock().unwrap()[rowindex].clone());
                 
                 }else if *table_data.total_order.lock().unwrap()[rowindex].payment=="1" {
                    *table_data.total_order.lock().unwrap()[rowindex].payment="2".to_string();
                    update_to_remote(table_data.total_order.lock().unwrap()[rowindex].clone());
                 }
                
           
                  
                }
            });
      
            toggle_row_selection(table_data,row_index, &row.response());
         
        });
       
        };
        body.row(20.0, |mut row| {
            if table_data.selection==table_data.total_order.lock().unwrap().len(){
                row.set_selected(true);
            }
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.order_number.concat()).size(20.0)).selectable(false),);
            });
           
        
        });
        
   
       
    });
    }
  
}  
fn toggle_row_selection(select:&mut TemplateApp, row_index: usize, row_response: &egui::Response) {
    if row_response.clicked() {
        
        if select.selection==row_index{
            let delete=select.total_order.lock().unwrap().remove(row_index);
        
            select.backup.push(delete.clone());
            select.logs.push(format!("order# {} Check out!@ {}", delete.clone().order_number, app::timenow()));
            save_to_remote( delete);
     
            select.selection=999;
        
        }else{
            select.selection=row_index
        }
    }

}
