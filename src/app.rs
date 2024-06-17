use chrono::{DateTime, Local};
use crossbeam_channel::unbounded;
use eframe::egui;
use egui::{ColorImage, Label, TextStyle, Ui};
use egui::{Id, RichText, TextureHandle, Vec2};
use egui_extras::RetainedImage;
use egui_extras::{Column, TableBuilder};
use ehttp::*;

use super::order_table::Order;

use super::order_table;
use super::order_table::save_to_remote;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
// use tokio::sync::mpsc::*;
// use tungstenite::Message;


use std::collections::VecDeque;


#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct LimitedVecDeque<T> {
    deque: VecDeque<T>,
    max_size: usize,
}

impl<T> LimitedVecDeque<T> {
    fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.deque.len() == self.max_size {
            self.deque.pop_front();
        }
        self.deque.push_back(value);
    }
    fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.deque.iter()
    }
    fn get(&self) -> &VecDeque<T> {
        &self.deque
    }
    fn len(&self) -> usize {
        self.deque.len()
    }
  
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Msg {
    vector: Vec<(String, String, String)>,
}

enum Download {
    None,
    Load,
    Done(),
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]

pub struct TemplateApp {
    // Example stuff:
    label: String,

    // This how you opt-out of serialization of a field
    pub order_number: Box<Vec<String>>,
    #[serde(skip)]
    pub total_order: Arc<Mutex<Vec<Order>>>,
    pub paid:  Box<Vec<String>>,
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
   pub height:f32,
    button:String,
    #[serde(skip)]
    pub logs:LimitedVecDeque<String>,
}
use serde_json::value::Serializer;
use serde_json::{from_slice, json, Deserializer};
    
    /*
fn buttons(template_app: &mut TemplateApp, ui: &mut Ui) {
    let wsize = ui.available_width();
    
    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
           for each in template_app.logs.iter(){
                
                ui.add_sized(
                    [wsize , 5.0],
                    egui::Label::new(each),   
                );
           
            }
    
        });
    
   
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
                template_app.height=5.0;
            }
        
         
           
        }
    });
    });
    
    ui.separator();
    if template_app.height!=5.0{

    ui.horizontal(|ui| {
        for but_index in 1..4 {
            let button = ui.add_sized(
                [wsize / 3.0 - 5.0,   template_app.height/4.0],
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
                [wsize / 3.0 - 5.0,   template_app.height/4.0],
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
                [wsize / 3.0 - 5.0,  template_app.height/4.0],
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
            [wsize / 3.0 - 5.0,  template_app.height/4.0],
            egui::Button::new("@".to_string()),
        );
        
        if button.clicked() {
           if template_app.selection<template_app.total_order.len(){
           
           
           let orderid= template_app.total_order[template_app.selection].order_number.clone();
           send_notification(orderid);
        }
        }
        let button_0 = ui.add_sized(
            [wsize / 3.0 - 5.0,  template_app.height/4.0],
            egui::Button::new("0".to_string()),
        );
        if button_0.clicked() {
            template_app.order_number.push("0".to_string());
            template_app.selection = 999;
            check_order(template_app);
        }
        let button_c = ui.add_sized(
            [wsize / 3.0 - 5.0,    template_app.height/4.0],
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
} */

use std::env;

use anyhow;
fn load_vector()-> anyhow::Result<Vec<Order>>{
    
    let request = Request{
        headers: ehttp::Headers::new(&[
            ("Content-Type", "application/json"),
        ]),..Request::get("https://127.0.0.1:3030/load")};
  
    let orders:Vec<Order>=Vec::new();
    
    ehttp::fetch(request, |response: Result<Response>| {
        match response {
            Ok(response) => {
                if response.status == 200 {
                    // Deserialize the JSON response into a vector
                    match from_slice::<Vec<Order>>(&response.bytes) {
                        Ok(order) => {
                            println!("Items: {:?}", order);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                        }
                    }
                } else {
                    eprintln!("HTTP error: {}", response.status);
                }
            }
            Err(e) => {
                eprintln!("Request failed: {}", e);
            }
        }
    });
   
   Ok(orders)
}
impl<'a> Default for TemplateApp {
    fn default() ->Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number: Box::new(Vec::new()),
            total_order:Arc::new(Mutex::new(Vec::new())),
          
            selection: 999,
            rows: 1,
            row_index: 0,
            friedbun_count: 0,
            payment: Default::default(),
            scroll_to_row: None,
            name: "".to_owned(),
            respond: Arc::new(Mutex::new(Download::Load)),
            paid: Box::new(Vec::new()),
            backup: Vec::new(),
            last_update: Utc::now().timestamp(),
            height:240.0,
            button:"🔻".to_string(),
            logs:LimitedVecDeque::new(5),
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
    retry : u64,
    expire : u64,
    called_back: String,
  
}

#[derive(Default)]
struct EchoServer;
pub fn timenow()->String{
    let time: DateTime<Local> = Local::now();
    let now = time.format("%H:%M").to_string();
    now
}
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
      /*   if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
         */
        Default::default()
    }
    fn check_for_database_updates(&mut self) {
      
        if Utc::now().timestamp()-self.last_update> 2 {
            self.last_update=Utc::now().timestamp();
          let mut totalorder= self.total_order.lock().unwrap();
          *totalorder=load_vector().unwrap();
        }
    } 
    fn restore_backup(&mut self){
        if let Some(backup) = self.backup.pop() {
            self.total_order.lock().unwrap().push(backup);
            self.logs.push(format!("restore last entry"));
            self.selection = self.total_order.lock().unwrap().len() - 1;
            self.scroll_to_row = Some(self.selection);
        
        }
       
    }
    fn restore_input(&mut self){
        if let Some(backup) = self.backup.pop() {
            self.total_order.lock().unwrap().push(backup);
            self.logs.push(format!("restore last entry"));
            self.selection = self.total_order.lock().unwrap().len() - 1;
            self.scroll_to_row = Some(self.selection);
        
        }
       
    }
    fn check_order(&mut self) {
        println!("Checking order: {:?}", self.order_number);
        
        if self.order_number.len() == 4 {
            println!("Order number length is 4");
            let backup_size = self.backup.len();
           
            let time: DateTime<Local> = Local::now();
            let now = time.format("%H:%M").to_string();
            
            if self.order_number.concat() == "0000" {
                println!("Order number is 0000");
                if backup_size > 0 {
                    if let Some(backup) = self.backup.pop() {
                        save_to_remote(backup.clone());
                        let target_first_value = backup.clone().order_number;
                        
                        let contains_first_value: Vec<_> = self
                            .total_order.lock().unwrap()
                            .iter() 
                            .enumerate()
                            .filter_map(|(index, item)| {
                                if item.order_number == target_first_value {
                                    Some(index)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        
                        if !contains_first_value.is_empty() {
                            for index in contains_first_value {
                                self.total_order.lock().unwrap().remove(index);
                                self.restore_backup();
                              
                            }
                            self.logs.push(format!("delete last entry"));
                        } else {
                            self.restore_input();
                        }
                    }
                    self.order_number.clear();
                    self.selection = 999;
                }
            } else {
                let target_first_value = self.order_number.concat();
                let contains_first_value: Vec<_> = self
                    .total_order
                    .lock().unwrap().iter()
                    .enumerate()
                    .filter_map(|(index, item)| {
                        if *item.order_number == target_first_value {
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .collect();
                
                if !contains_first_value.is_empty() {
                    for index in contains_first_value {
                        
                        self.delete(index);
                 
                    }
                
                } else {
                    let order = Order {
                        order_number:Box::new(self.order_number.concat()),
                        check_in: Box::new(time.to_rfc3339()),
                        payment: Box::new("0".to_owned()),
                    };

                    save_to_remote(order.clone());
                   self.total_order.lock().unwrap().push(order.clone());
                   self.logs.push(format!("order# {} Check in!@ {}", order.order_number, now));
               
                    self.selection = self.total_order.lock().unwrap().len() - 1;
                    self.scroll_to_row = Some(self.selection);
                }
                if backup_size > 10 {
                    self.backup.clear();
                }
    
            }
            self.order_number.clear();
        }
    }         
    

pub fn send_notification(order_numer:String) {
         
    let data =Notify {
        token:"asw1cdy52m362zu57zxua7efgjofyh".to_string(),
        user: "u8skjyeb16zq9hmq2jg1gbtmazw485".to_string(),
        title: "Order Checkin!".to_string(),
        message: format!("order: {} Customer CheckIn!",order_numer),
        priority :"1".to_string(),
        retry : 30,
        expire: 150,
        called_back: "https://settingupdate.com/new/callback.php".to_string(),
   
      
      };

    let request = Request{
        headers: ehttp::Headers::new(&[
            ("Content-Type", "application/json"),
        ]),..Request::json("https://api.pushover.net/1/messages.json",&data).unwrap()};
   
    ehttp::fetch(request, move |response| {
        
        println!("Response: {:?}",response);
    });


}    
    
    pub fn delete(&mut self,index:usize){
        

        let order=self.total_order.lock().unwrap().remove(index);
        order_table::save_to_remote(order.clone());
        self.backup.push(order.clone());
        self.logs.push(format!("order# {} Check out!@ {}", order.order_number,timenow()));
       
    
    }
    fn render_order_buttons(&mut self, ui: &mut Ui) {

        let wsize = ui.available_width();
    
      
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            
               for each in self.logs.iter(){
                    
                    ui.add_sized(
                        [wsize, 5.0],
                        egui::Label::new(each),   
                    );
               
                }
        
       
            });
   
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            
          
            let button_down = ui.add_sized(
                [wsize / 6.0 - 5.0, 3.0],
                egui::Button::new(self.button.to_string()),
            );
            if button_down.clicked() {
                if   self.button=="🔺".to_string(){
                    self.button="🔻".to_string();
                    self.height=240.0;
                }else{
                    self.button="🔺".to_string();
                    self.height=5.0;
                }
            
             
               
            }
        });
   
        
        ui.separator();
        if self.height!=5.0{

        let wsize = ui.available_width();
        
        for row in 0..3 {
            ui.horizontal(|ui| {
                for col in 1..4 {
                    let but_index = row * 3 + col;
                    if but_index < 10 {
                      
                        let button = ui.add_sized(
                            [wsize / 3.0 - 5.0, self.height / 4.0],
                            egui::Button::new(but_index.to_string()),
                        );
                   
                        if button.clicked() {
                            self.selection = 999;
                            self.order_number.push(but_index.to_string());
                            self.check_order();
                        } 
                    }
                }
            });
        }

        ui.horizontal(|ui| {
            self.render_control_buttons(ui, wsize);
        });
    }
    }
    
    fn render_control_buttons(&mut self, ui: &mut Ui, wsize: f32) {
        
        
        let button = ui.add_sized(
            [wsize / 3.0 - 5.0, self.height / 4.0],
            egui::Button::new("@".to_string()),
        );
        if button.clicked() {
            if self.selection < self.total_order.lock().unwrap().len() {
                let orderid = self.total_order.lock().unwrap()[self.selection].order_number.clone();
                TemplateApp::send_notification(*orderid);
            }
        }
        
        let button_0 = ui.add_sized(
            [wsize / 3.0 - 5.0, self.height / 4.0],
            egui::Button::new("0".to_string()),
        );
        if button_0.clicked() {
            self.order_number.push("0".to_string());
            self.selection = 999;
            self.check_order();
        }
        
        let button_c = ui.add_sized(
            [wsize / 3.0 - 5.0, self.height / 4.0],
            egui::Button::new("C".to_string()),
        );
        if button_c.clicked() {
            self.button_c_clicked()
           
        }
    }

    fn button_c_clicked(&mut self){
       
        if self.selection != 999 {
            if self.selection<self.total_order.lock().unwrap().len(){
        
            self.delete( self.selection);
          
            self.update_selection();
        }
        } else {
            self.order_number.clear();
        }

    }
    fn update_selection(&mut self) {
        if self.selection == self.total_order.lock().unwrap().len() && !self.total_order.lock().unwrap().is_empty() {
            self.selection = self.total_order.lock().unwrap().len() - 1;
        } else {
            self.selection = 999;
        }
    }
    fn handle_network_response(&mut self, response: ehttp::Result<ehttp::Response>) {
        match response {
            Err(err) => {
              
            }
            Ok(response) => {
                self.respond = Arc::new(Mutex::new(Download::None));
                let text = response.text().unwrap_or_default();
                let orders: Vec<Order> = serde_json::from_str(&text).expect("JSON was not well-formatted");
                self.total_order =Arc::new(Mutex::new(orders));
            }
        }
    }

}



impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
   
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
   
        egui::CentralPanel::default().show(ctx, |ui| {
            let ctx = ctx.clone();
       
       
       

            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            use egui_extras::{Size, StripBuilder};
            StripBuilder::new(ui)
                .size(Size::remainder()) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            let mut table = Box::new(order_table::Table::default());
                            table.table_ui(ui, self);
                        });
                    });
                });
            
         
          
        });
        
      
        egui::TopBottomPanel::bottom("bot").show(ctx, |ui|

        self.render_order_buttons(ui));
     

        let copy = &self.respond.clone();
            let download: &Download = &copy.lock().unwrap();
            
            let respond_store = self.respond.clone();
        self.check_for_database_updates(); 
        match download {
           Download::None => {}
           Download::Load => {
            let mut totalorder= self.total_order.lock().unwrap();
            *totalorder=load_vector().unwrap();
            *respond_store.lock().unwrap()=Download::None;
                   }
           Download::Done() => {
            let mut totalorder= self.total_order.lock().unwrap();
            *totalorder=load_vector().unwrap();
                   
           }
           , 
       }
        ctx.request_repaint();
    
    }
    
}
