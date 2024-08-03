#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
extern crate ct_nox;
use ct_nox::ct_nox::{read_file, write_file};
use ct_nox::encrypt::{encrypt};
use ct_nox::decrypt::{decrypt};
use eframe::egui;
use ct::icon::{get_icon};
use egui::TextBuffer as _;
//use native_dialog::{MessageDialog, MessageType};
//use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
	drag_and_drop_support: true,
	icon_data: Some(load_icon()),	
	initial_window_size: Some(egui::vec2(320.0, 240.0)),
	..Default::default()
    };

    eframe::run_native(
        "CT",
        options,
        Box::new(|_cc| Box::new(Ct::default())),
    )
}


#[derive(Default)]
struct Ct {
    text: String,
    picked_path: String,
    cursor1: usize,
    cursor2: usize,
    password: String,
    window_help_open: bool,
    window_about_open: bool,    
}


// https://www.egui.rs/#demo
// https://docs.rs/egui/latest/egui/
// https://docs.rs/egui/latest/egui/widgets/text_edit/struct.TextEdit.html
impl eframe::App for Ct {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.window_help_open {
            egui::Window::new("Help")
                .open(&mut self.window_help_open)
                .show(ctx, |ui| {
                    ui.label("contents");
                });
        }

        if self.window_about_open {
            egui::Window::new("About")
                .open(&mut self.window_about_open)
                .show(ctx, |ui| {
                    ui.label("contents");
                });
        }        

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
                let _password = ui.add(egui::TextEdit::singleline(&mut self.password)
				       .hint_text("Please enter your password")
				       .desired_width(f32::INFINITY)
				       .password(true)				       
		);
	        let _scroll = egui::ScrollArea::vertical().show(ui, |ui| {
                let textedit = egui::TextEdit::multiline(&mut self.text)
		    .desired_width(f32::INFINITY)
		    .hint_text("Please enter your text");

		let response = ui.add_sized(ui.available_size(), textedit);
                //https://docs.rs/egui/0.21.0/egui/struct.Response.html#method.hovered
		let resp_id = response.id;
		if let Some(state) = egui::TextEdit::load_state(ui.ctx(), resp_id)
		{
		    if let Some(ccursor) = state.ccursor_range()
		    {
                      self.cursor1 = ccursor.secondary.index;
		       self.cursor2 = ccursor.primary.index;
		    }
		}   
            });

        });

	//https://docs.rs/egui/latest/egui/struct.Ui.html
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Crypt Text").clicked() {
			if let Some(path) = rfd::FileDialog::new().pick_file()
			{
			  self.picked_path = path.display().to_string();
		          let ct  = read_file(&self.picked_path.clone());
                          self.text = decrypt(&ct,&self.password);
                        }
                        ui.close_menu();                      
                    }


		    
                    if ui.button("Save As Crypt Text").clicked() {
			if let Some(path) = rfd::FileDialog::new().save_file()
			{
  		            self.picked_path = path.display().to_string();
			    println!("save crypt text to: {}",self.picked_path);
			    let ct = encrypt(&self.text,&self.password);
			    let _x  = write_file(&self.picked_path.clone(),&ct);
                        }
                        ui.close_menu();
                    }		    

                    if ui.button("Open Text").clicked() {
			if let Some(path) = rfd::FileDialog::new().pick_file()
			{
			  self.picked_path = path.display().to_string();
		          self.text  = read_file(&self.picked_path.clone());

                        }
                        ui.close_menu();                      
                    }

                    if ui.button("Save As Text").clicked() {
			if let Some(path) = rfd::FileDialog::new().save_file()
			{
  		            self.picked_path = path.display().to_string();
			    println!("save text to: {}",self.picked_path);
			    let _x  = write_file(&self.picked_path.clone(),&self.text);
                        }
                        ui.close_menu();
                    }

                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
		      let r = get_char_range(self.cursor1,self.cursor2);
                      let st = self.text.char_range(r.clone());			
                      ui.output_mut(|o| o.copied_text = st.to_string());                        
		      self.text.delete_char_range(r.clone());			
                      ui.close_menu();                        
                    }
                    if ui.button("Copy").clicked() {
		      let r = get_char_range(self.cursor1,self.cursor2);
                      let st = self.text.char_range(r.clone());			
                      ui.output_mut(|o| o.copied_text = st.to_string());
                      ui.close_menu();                        			
                    }
                    if ui.button("Paste").clicked() {
		      let txt = cli_clipboard::get_contents().unwrap();
    	              let r = get_char_range(self.cursor1,self.cursor2);
                      self.text.insert_text(&txt,r.start);
                      ui.close_menu();                        						
                    }

                    if ui.button("Encrypt").clicked() {
		      let r = get_char_range(self.cursor1,self.cursor2);
                      let st = self.text.char_range(r.clone());			
		      let ct = encrypt(&st, &self.password);
                      self.text.delete_char_range(r.clone());
                      self.text.insert_text(&ct,r.start);
                      ui.close_menu();
                    }

                    if ui.button("Decrypt").clicked() {
		      let r = get_char_range(self.cursor1,self.cursor2);
                      let ct = self.text.char_range(r.clone());			
		      let txt = decrypt(&ct, &self.password);
                      self.text.delete_char_range(r.clone());
                      self.text.insert_text(&txt,r.start);
                      ui.close_menu();
                    }		    

                    //if ui.button("Preferences").clicked() {
                    //      println!("preferences....");
                    //}		    		    
                });


                ui.menu_button("Help", |ui| {
                    if ui.button("Help").clicked() {
                       self.window_help_open = true;
                       ui.close_menu();
                    }
                    if ui.button("About").clicked() {
                       self.window_about_open = true;
                       ui.close_menu();                        
                    }

                });

		
		
            })
        });

	
    }
}


fn get_char_range(c1:usize, c2:usize)-> std::ops::Range<usize>
{
  //https://docs.rs/egui/latest/egui/widgets/text_edit/trait.TextBuffer.html#method.char_range
  let mut a = c1;
  let mut b = c2;
  if a > b
  {
    a = c2;
    b = c1;
  }
  let r = std::ops::Range { start: a, end: b };
  return r;
}
    
fn load_icon() -> eframe::IconData
{
  let (icon_rgba, icon_width, icon_height) = {
    let rgba = get_icon();
    (rgba, 64, 64)
  };
    
  eframe::IconData {
    rgba: icon_rgba,
    width: icon_width,
    height: icon_height,
  }
}



//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}
