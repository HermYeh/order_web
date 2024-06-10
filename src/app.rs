use chrono::{DateTime, Local};
use crossbeam_channel::unbounded;
use eframe::egui;
use egui::{ColorImage, Label, TextStyle, Ui};
use egui::{Id, RichText, TextureHandle, Vec2};
use egui_extras::RetainedImage;
use egui_extras::{Column, TableBuilder};
use ehttp::*;
use image;
use std::fs;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::thread;
use super::order_table::Order;

use super::order_table;
use super::order_table::save_to_remote;

use chrono::Utc;
use serde::{Deserialize, Serialize};






use std::sync::Arc;
// use tokio::sync::mpsc::*;
// use tungstenite::Message;



#[derive(Serialize, Deserialize, Clone, Debug)]
struct Msg {
    vector: Vec<(String, String, String)>,
}

enum Download {
    None,
    Load,
    Done(ehttp::Result<ehttp::Response>),
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]

pub struct TemplateApp {
    // Example stuff:
    label: String,

    // This how you opt-out of serialization of a field
    pub order_number: Vec<String>,
    #[serde(skip)]
    pub total_order: Vec<Order>,
    pub order_time: Vec<String>,
    pub paid: Vec<String>,
    pub selection: usize,
    rows: i32,
    row_index: i32,
    friedbun_count: i32,
    pub payment: Vec<bool>,
    pub scroll_to_row: Option<usize>,
    name: String,
    #[serde(skip)]
    respond: Arc<Mutex<Download>>,
    pub backup: Vec<Order>,
    #[serde(skip)]
    last_update: i64,
    height:f32,
    button:String,
    logs:Vec<String>,
}
use serde_json::value::Serializer;
use serde_json::Deserializer;
fn check_order(template_app: &mut TemplateApp) {
    
   
    if template_app.order_number.len() == 4 {
        let backup_size=template_app.backup.len();
        let backup_data: Order;
        if template_app.order_number.concat() == "0000" {
            if backup_size>0{
            let backup = template_app.backup.pop();
            match backup {
                Some(data) => {
                    backup_data = data.clone();

                    save_to_remote(data);
                    let target_first_value = backup_data.clone().order_number;
                    
                    
                   
                    let contains_first_value: Vec<_> = template_app
                        .total_order
                        .iter()
                        .enumerate()
                        .filter_map(|(index,item)| {
                            if item.order_number == target_first_value {
                                Some(index)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if !contains_first_value.is_empty() {
                        for index in contains_first_value {
                            template_app.total_order.remove(index);
                        }
                        template_app.order_number.clear();
                    } else {
                        template_app.total_order.push(backup_data.clone());
                        template_app.order_number.clear();
                        template_app.selection = template_app.total_order.len() - 1;
                        template_app.scroll_to_row = Some(template_app.selection);
                    }
                }
                None => {}
            };

            template_app.order_number.clear();
            template_app.selection = 999;
            }
        } else {
            
          
            let target_first_value = template_app.order_number.concat();
            let contains_first_value: Vec<_> = template_app
                .total_order
                .iter()
                .enumerate()
                .filter_map(|(index,item)| {
                    if item.order_number == target_first_value {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect();
            if !contains_first_value.is_empty() {
                for index in contains_first_value {
                    template_app
                        .backup
                        .push(template_app.total_order[index].clone());
                    template_app.logs.push(format!("order# {} Checkout!",template_app.total_order[index].clone()));
                    order_table::save_to_remote(template_app.total_order.remove(index));
                }
                template_app.order_number.clear();
          
            } else {
                let time: DateTime<Local> = Local::now();
                println!("{}", time.to_rfc3339().to_string());
                let order = Order{
                    order_number: template_app.order_number.concat(),
                    check_in: time.to_rfc3339(),
                    payment: "0".to_owned(),
                
                   
                };
                template_app.logs.push(format!("order# {} Checkin!",order.order_number));
                template_app.backup.push(order.clone());
                order_table::save_to_remote(order.clone());
                template_app.total_order.push(order);
                template_app.order_number.clear();
                template_app.selection = template_app.total_order.len() - 1;
                template_app.scroll_to_row = Some(template_app.selection);
            }
            if backup_size>10{
                template_app.backup.clear();
            }
        }


    };
}                                                                   
fn buttons(template_app: &mut TemplateApp, ui: &mut Ui) {
    let wsize = ui.available_width();
    
   
            for each in &template_app.logs{
                
                ui.add_sized(
                    [wsize / 6.0 - 5.0, 5.0],
                    egui::Label::new(each),   
                );
            }
    
    

    let height= template_app.height;
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
        
      
        let button_down = ui.add_sized(
            [wsize / 6.0 - 5.0, 5.0],
            egui::Button::new(template_app.button.to_string()),
        );
        if button_down.clicked() {
            if   template_app.button=="🔺".to_string(){
                template_app.button="🔻".to_string();
                template_app.height=240.0;
            }else{
                template_app.button="🔺".to_string();
                template_app.height=4.0;
            }

         
           
        }
    });
    });
    if template_app.height!=4.0{

    ui.horizontal(|ui| {
        for but_index in 1..4 {
            let button = ui.add_sized(
                [wsize / 3.0 - 5.0, height/4.0],
                egui::Button::new(but_index.to_string()),
            );
            if button.clicked() {
                template_app.selection = 999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
    });
    ui.horizontal(|ui| {
        for but_index in 4..7 {
            let button = ui.add_sized(
                [wsize / 3.0 - 5.0, height/4.0],
                egui::Button::new(but_index.to_string()),
            );

            if button.clicked() {
                template_app.selection = 999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
    });
    ui.horizontal(|ui| {
        for but_index in 7..10 {
            let button = ui.add_sized(
                [wsize / 3.0 - 5.0,height/4.0],
                egui::Button::new(but_index.to_string()),
            );
            if button.clicked() {
                template_app.selection = 999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
    });
    ui.horizontal(|ui| {
        let button = ui.add_sized(
            [wsize / 3.0 - 5.0,height/4.0],
            egui::Button::new("@".to_string()),
        );
        
        if button.clicked() {
           if template_app.selection<template_app.total_order.len(){
           
           
           let orderid= template_app.total_order[template_app.selection].order_number.clone();
           send_notification(orderid);
        }
        }
        let button_0 = ui.add_sized(
            [wsize / 3.0 - 5.0,height/4.0],
            egui::Button::new("0".to_string()),
        );
        if button_0.clicked() {
            template_app.order_number.push("0".to_string());
            template_app.selection = 999;
            check_order(template_app);
        }
        let button_c = ui.add_sized(
            [wsize / 3.0 - 5.0, height/4.0],
            egui::Button::new("C".to_string()),
        );
        if button_c.clicked() {
            if template_app.selection != 999 {
                template_app
                    .backup
                    .push(template_app.total_order[template_app.selection].clone());
                order_table::save_to_remote(
                    template_app.total_order.remove(template_app.selection),
                );
                if template_app.selection == template_app.total_order.len()
                    && template_app.total_order.len() > 0
                {
                    template_app.selection = template_app.total_order.len() - 1;
                } else {
                    template_app.selection = 999;
                }
            } else {
                template_app.order_number.clear();
            }
        }
    });
    }
}

impl<'a> Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number: Vec::new(),
            total_order: Default::default(),
            order_time: Vec::new(),
            selection: 999,
            rows: 1,
            row_index: 0,
            friedbun_count: 0,
            payment: Default::default(),
            scroll_to_row: None,
            name: "".to_owned(),
            respond: Arc::new(Mutex::new(Download::Load)),
            paid: Vec::new(),
            backup: Vec::new(),
            last_update: Utc::now().timestamp(),
            height:240.0,
            button:"🔻".to_string(),
            logs:Vec::new(),
        }
    }
}


#[derive(Debug, Copy, Clone)]
#[repr(C, align(8))]
struct FileHeader {
    size: u32,
}

const BUF_LEN: usize = 4096;
use std::mem::size_of;
use std::slice;
use std::sync::Mutex;
#[derive(Serialize, Deserialize, Clone, Debug,PartialEq,Eq)]
struct Notify {
    token:String,
    user:String,
    title:String,
    message:String,
    priority :String,
}

fn load_vector(respond_store: Arc<Mutex<Download>>) {
    
    
    let request = Request::get("https://settingupdate.com/new/load.php");
    ehttp::fetch(request, move |response| {
        *respond_store.lock().unwrap() = Download::Done(response);
    });


}
fn send_notification(order_numer:String) {
         
    let data =Notify {
        token:"asw1cdy52m362zu57zxua7efgjofyh".to_string(),
        user: "u8skjyeb16zq9hmq2jg1gbtmazw485".to_string(),
        title: "Order Checkin!".to_string(),
        message: format!("order: {} Customer CheckIn!",order_numer),
        priority :"2".to_string(),
      };
    let request = Request{
        headers: ehttp::Headers::new(&[
            ("Content-Type", "application/json"),
        ]),..Request::json("https://api.pushover.net/1/messages.json",&data).unwrap()};
   
    ehttp::fetch(request, move |response| {
    
    });


}
#[derive(Default)]
struct EchoServer;

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    fn check_for_database_updates(&mut self) {
      
        if Utc::now().timestamp()-self.last_update> 2 {
            self.last_update=Utc::now().timestamp();
            load_vector(self.respond.clone());
        }
    } 
}



impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
     
        egui::CentralPanel::default().show(ctx, |ui| {
            let ctx = ctx.clone();
             self.check_for_database_updates(); 
       
           
            /*  let respond_store = self.respond.clone();


            let getrequest = Request::get("https://ts.maya.se/1.php");


            ehttp::fetch(getrequest, move |response| {

                if response.unwrap().text()==Some("1"){

                    load_vector(respond_store);
                }

            }); */

            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            use egui_extras::{Size, StripBuilder};
            StripBuilder::new(ui)
                .size(Size::remainder()) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            let mut table = order_table::Table::default();
                            table.table_ui(ui, self);
                        });
                    });
                });

            let copy = &self.respond.clone();
            let download: &Download = &copy.lock().unwrap();
            
            let respond_store = self.respond.clone();

            match download {
                Download::None => {}
                Download::Load => {
                    load_vector(respond_store);
                }
                Download::Done(response) => match response {
                    Err(err) => {
                        ui.label(err);
                    }
                    Ok(response) => {
                        self.respond = Arc::new(Mutex::new(Download::None));
                        let text = &response.text();
                        println!("{}", text.unwrap());
                        let orders: Vec<Order> = serde_json::from_str(text.unwrap())
                            .expect("JSON was not well-formatted");
                        println!("{:?}", &orders);
                        self.total_order=orders;
                 
                       /*  self.total_order.clear();
                        for order in orders {
                            self.total_order.push((
                                order.order_number,
                                order.check_in,
                                order.payment,
                            ));
                            println!("{:?}", &self.total_order);
                        } */
                    }
                },
            }
        });
        
        egui::TopBottomPanel::bottom("bot").show(ctx, |ui|
        
             buttons(self, ui));
        
        
        ctx.request_repaint();

    }
}
