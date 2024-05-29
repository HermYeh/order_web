
use egui::{ColorImage,Label, TextStyle, Ui};
use ehttp::*;
use crossbeam_channel::unbounded;
use std::fs;
use egui_extras::RetainedImage;
use std::time::Duration;
use egui_extras::{TableBuilder, Column};
use chrono::{DateTime, Local};
use std::path::Path;
use eframe::{egui};
use egui::{ Id, RichText, TextureHandle, Vec2};
use image;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::io::ErrorKind;
use std::thread;
use std::io::Read;
use std::io::Write;


use crate::order_table;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Clone,Debug)]
struct Msg {
    vector:Vec<(String,String,String)>,
}

enum Download {
    None,
    Load,
    Done(ehttp::Result<ehttp::Response>),
}

#[derive(serde::Deserialize, serde::Serialize,Clone)]
#[serde(default)]

pub struct TemplateApp {
    // Example stuff:
    label: String,
    
// This how you opt-out of serialization of a field
    pub order_number: Vec<String>,
    #[serde(skip)]
    pub total_order:Vec<(String, String,String)>,
    pub order_time:Vec<String>,
    pub paid:Vec<String>,
    pub selection: usize,
    rows: i32,
    row_index: i32,
    friedbun_count: i32,
    pub payment: Vec<bool>,
    pub scroll_to_row: Option<usize>,
    name:String,
    #[serde(skip)]
    respond: Arc<Mutex<Download>>,
}
use serde_json::value::Serializer;
use serde_json::Deserializer;
fn check_order(template_app:&mut TemplateApp){
     
    if template_app.order_number.len()==4{
        let target_first_value =&template_app.order_number.concat();
        let contains_first_value : Vec<_>= template_app.total_order.iter().enumerate()
        .filter_map(|(index,(first, _,_))|{if first == target_first_value {
                Some(index)
            } else {
                None
            }
        })
        .collect();
        if !contains_first_value.is_empty() {
          
            for index in contains_first_value {
               
                order_table::save_to_remote(vec![template_app.total_order.remove(index)]);
                
            }
            template_app.order_number.clear();
            
        } else {
            let time: DateTime<Local> = Local::now();
            println!("{}",time.to_rfc3339().to_string());
            let order=(template_app.order_number.concat(),time.to_rfc3339(),"0".to_owned());
          
            order_table::save_to_remote(vec![order.clone()]);
            template_app.total_order.push(order);
            template_app.order_number.clear();
            
        }
     
      
    };
}
fn buttons(template_app:&mut TemplateApp,ui:&mut Ui){
     let wsize=ui.available_width();
     ui.vertical(|ui| {
               
    ui.horizontal(|ui| {     
            
        for but_index in 1..4{
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new(but_index.to_string())
            ) ;
            if button.clicked(){
                template_app.selection=999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });
    ui.horizontal(|ui| {     
        for but_index in 4..7{
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new(but_index.to_string())
            ) ;

            if button.clicked(){
                template_app.selection=999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });  
    ui.horizontal(|ui| {     
        for but_index in 7..10{
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new(but_index.to_string())
            ) ;
            if button.clicked(){
                template_app.selection=999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });    
    ui.horizontal(|ui| {     
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new("@".to_string())
            ) ;

            if button.clicked(){
                let respond_store = template_app.respond.clone();
                load_vector(respond_store);
            }
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new("0".to_string())
            ) ;
            if button.clicked(){
                template_app.order_number.push("0".to_string());
                template_app.selection=999;
                check_order(template_app);
            }
            let button_c = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new("C".to_string())
            ) ;
            if button_c.clicked(){
                if template_app.selection!=999{
                template_app.total_order.remove(template_app.selection);
                template_app.payment.remove(template_app.selection);
                template_app.payment.push(false);
                template_app.selection=999;
                }else{
                    template_app.order_number.clear();
                }
                order_table::save_to_remote(template_app.total_order.clone());
            }
    });  
});  
}

impl<'a> Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number:Vec::new(),
            total_order:Default::default(),
            order_time:Vec::new(),
            selection: 999,
            rows: 1,
            row_index: 0,
            friedbun_count: 0,
            payment: vec![false;50],
            scroll_to_row: None,
            name:"".to_owned(),
            respond: Arc::new(Mutex::new(Download::Load)),
            paid:Vec::new(),
        }
    }
  
}

use std::{
    net::TcpStream,
    sync::mpsc,
};

#[derive(Debug, Copy, Clone)]
#[repr(C, align(8))]
struct FileHeader {
    size: u32,
    
}


const BUF_LEN: usize = 4096;
use std::slice;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::Mutex;

fn load_vector(respond_store: Arc<Mutex<Download>>)
{  
    let request = Request::get("https://settingupdate.com/new/load.php");
               ehttp::fetch(request, move |response| {
                *respond_store.lock().unwrap()=Download::Done(response);
          
                });
              
 
    
  
}

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
   
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    order_number: String,
    check_in: String,
    payment:String,
}
use std::{net::TcpListener, thread::spawn};

use std::env;


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
                            let mut table=order_table::Table::default();
                            table.table_ui(ui,self);
                        });
                    });
                  
                });
           
               let copy= &self.respond.clone();
                let download: &Download=&copy.lock().unwrap();
                
          
            let respond_store = self.respond.clone();
           
                match download {
                    Download::None => {}
                    Download::Load=>{
                    
                    
                        load_vector(respond_store);
                      
                    }
                    Download::Done(response) => match response {
                        Err(err) => {
                            ui.label(err);
                        }
                        Ok(response) => {
                            self.respond=Arc::new(Mutex::new(Download::None));
                            let text=  &response.text();
                            println!("{}", text.unwrap());
                            let orders: Vec<Order> = serde_json::from_str(text.unwrap()).expect("JSON was not well-formatted");
                            println!("{:?}", &orders);
                          self.total_order.clear();
                            for order in orders{
                       
                                self.total_order.push((order.order_number,order.check_in, order.payment));
                                println!("{:?}", &self.total_order);
                            }
                            
                        
                           
                        }
                    },
                }
            
            
        });
     
  
        
        egui::TopBottomPanel::bottom("bot").show(ctx, |ui| {
  
      
            buttons(self, ui)
    
      
    });
    ctx.request_repaint();
  
    }
}


