use chrono::{DateTime, FixedOffset, Local};

use egui::{Color32, Label, Stroke, TextStyle};
use chrono::TimeZone;
use ehttp::Request;
use std::io::Bytes;
use crate::TemplateApp;

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
pub fn save_to_remote(total_order:Vec<(String, String,String)>)  {
 
    let output=Msg{
        vector:total_order.clone(),
    };
  
    
  
    println!("my_data: {:?}",total_order);
   
     let request = Request{
        headers: ehttp::Headers::new(&[
            ("Content-Type", "application/json"),
        ]),..Request::json("https://settingupdate.com/new/order.php",&output).unwrap()};
  
    
        ehttp::fetch(request, move |response| {
 
            println!("Response: {:?}",response);
            
         });
     


}

impl Table {
    
   pub fn table_ui(&mut self, ui: &mut egui::Ui,table_data:&mut TemplateApp) {
        use egui_extras::{Column, TableBuilder};

   
    let s = Stroke {
            width: 0.0,
            color: Color32::TRANSPARENT,
        };
    let available_height = ui.available_height()-250.00;
    let wsize=ui.available_width();
    let mut table = TableBuilder::new(ui).cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
        .column(Column::auto())
        .column(Column::exact(wsize/4.00))
        .column(Column::exact(wsize/4.00))
        .column(Column::exact(wsize/4.00))
        .resizable(false)
        .striped(self.striped)
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);
        
        table = table.sense(egui::Sense::click());
        if let Some(row_index) = table_data.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, None);
        }
    
                
        
    table.header(20.0, |mut header| {
        header.col(|ui| {
         
        let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Order#").fill(egui::Color32::TRANSPARENT));
        if sort_click.clicked(){
            let data_temp= table_data.total_order.clone();
            table_data.total_order.sort();
            if data_temp== table_data.total_order{
                table_data.total_order.reverse();
            }
          
        }
        
        });
        header.col(|ui| {
            let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Check In").fill(egui::Color32::TRANSPARENT));
            if sort_click.clicked(){
                let data_temp= table_data.total_order.clone();
                table_data.total_order.sort_by_key(|k| DateTime::parse_from_rfc3339(&k.1).unwrap());
                if data_temp== table_data.total_order{
                    table_data.total_order.reverse();
                }
              
            }
         

        });
        header.col(|ui| {
            let sort_click=ui.add_sized(ui.available_size(),egui::Button::new("Wait Time").fill(egui::Color32::TRANSPARENT));
            if sort_click.clicked(){
                let data_temp= table_data.total_order.clone();
                table_data.total_order.sort_by_key(|k| DateTime::parse_from_rfc3339(&k.1).unwrap());
                if data_temp== table_data.total_order{
                    table_data.total_order.reverse();
                }
              
            }
        });
        header.col(|ui| {
          ui.add_sized(ui.available_size(),egui::Button::new("Payment").fill(egui::Color32::TRANSPARENT));
     
        })
        ;
        })
        
    .body(|mut body| {
    let order_size=table_data.total_order.len();
    for row_index in 0..order_size {    
        if row_index>=table_data.total_order.len(){
            continue;
         }
        body.row(20.0, |mut row| {
            let rowindex=row.index();
            if table_data.selection==row_index{
                row.set_selected(true);
            }else {
                row.set_selected(false);
            }
            
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.total_order[rowindex].0.clone()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(DateTime::parse_from_rfc3339(&table_data.total_order[rowindex].1.clone()).unwrap().format("%H:%M").to_string()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let time_now: DateTime<Local> = Local::now();
                let time_wait = time_now.to_utc()-(DateTime::parse_from_rfc3339(&table_data.total_order[rowindex].1.clone()).unwrap().to_utc());
        
             
                let minutes = (time_wait.num_minutes()).to_string();
               
               
               let time= minutes+" min";
                ui.add_sized( ui.available_size(),Label::new(egui::RichText::new(time).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let response = ui
                .add_sized(
                    ui.available_size(),
                    egui::Button::new(if table_data.total_order[rowindex].2=="1" {"Paid"}else{""}),
                );
                if response.clicked(){
                 if table_data.total_order[rowindex].2=="1"{

                    table_data.total_order[rowindex].2="0".to_owned();
                 }else{
                    table_data.total_order[rowindex].2="1".to_owned();
                 }

              
                  
                }
            });
            
            toggle_row_selection(table_data,row_index, &row.response());
         
        });
       
        };
        body.row(20.0, |mut row| {
            if table_data.selection==table_data.total_order.len(){
                row.set_selected(true);
            }else {
                row.set_selected(false);
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
        select.selection=row_index;
    }
    if row_response.double_clicked() {
     
        select.total_order.remove(row_index);
        select.payment.remove(row_index);
        select.payment.push(false);
        select.selection=999;
     save_to_remote(select.total_order.clone());
    
    }
}
