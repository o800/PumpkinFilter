use eframe::egui;
use curl::easy::Easy;
use eframe::emath::Align;
use egui::{Layout, OpenUrl};
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use egui::{RichText, Color32};
use egui::TextEdit;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([306f32, 490f32]),
        ..Default::default()
    };
    eframe::run_native("PumpkinFilter", native_options, Box::new(|cc| Ok(Box::new(PumpkinFilter::new(cc)))));
}

#[derive(Default)]
struct PumpkinFilter {
    input: String,
    claimed: String,
    pumpkins: String,
}

#[derive(Serialize, Deserialize)]
struct Claimed {
    claimed: Vec<u32>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct TileData {
    lat: f64,
    lng: f64,
    tileX: i32,
    tileY: i32,
    offsetX: i32,
    offsetY: i32,
    foundAt: String,
}


impl PumpkinFilter {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.storage;
        Self::default()
    }
}

impl eframe::App for PumpkinFilter {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            ui.heading(RichText::new("Wplace Pumpkin Filter").color(Color32::ORANGE));

            ui.label("raw data from wplace.samuelscheit.com");

            ui.separator();
            ui.add_space(7.0);

            //ui.label(ctx.viewport_rect().max.to_string());



            if self.pumpkins.is_empty() {
                self.pumpkins = fetch_list()
            }

            /*ui.text_edit_singleline(&mut self.input);
            if ui.button("set claimed list").clicked() {
                self.claimed = self.input.clone();
                self.input.clear();
            } */
            //ui.heading(self.claimed.as_str());

            let textfield = ui.add(TextEdit::singleline(&mut self.input).hint_text("filter list"));
            if textfield.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                /*self.claimed = self.input.clone();
                self.input.clear();*/
                self.claimed = update_claimed(self.claimed.clone(), self.input.clone());
                self.input.clear();
            }

            ui.horizontal(|ui| {
                ui.add_space(3.0);
                if ui.button(RichText::new("Your Claimed Pumpkins").color(Color32::ORANGE)).clicked() {
                    ui.ctx().open_url(OpenUrl::new_tab("https://backend.wplace.live/event/hallowen/pumpkins/claimed"))
                }
                /*if ui.button("Submit").clicked() {
                    self.claimed = self.input.clone();
                    self.input.clear();
                }*/

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(7.0);
                    if ui.button(RichText::new("Update Pumpkin List").color(Color32::ORANGE)).clicked() {
                        if self.input.contains("{\"claimed\":[") {
                            self.claimed = self.input.clone();
                            self.input.clear();
                        } else if !self.input.is_empty() && self.claimed.contains("]}") && self.claimed.contains("[") {
                            self.claimed.pop();
                            self.claimed.pop();
                            let tempchar = self.claimed.pop().unwrap();
                            let char = tempchar.clone();
                            self.claimed.push(tempchar);
                            if char == '[' {
                                self.claimed += self.input.replace(" ", ",").as_str();
                            } else {
                                self.claimed.push(',');
                                self.claimed += self.input.replace(" ", ",").as_str();
                            }
                            self.claimed.push(']');
                            self.claimed.push('}');
                        } else if !self.input.is_empty() {
                            self.claimed = format!("{0}\"claimed\":[{1}]{2}", "{", self.input.replace(" ", ",").as_str(), "}" );
                        }
                        self.input.clear();
                        self.pumpkins = fetch_list()
                    }
                });
            });

            ui.separator();
            ui.add_space(7.0);



            let mut claimed: Claimed = serde_json::from_str(self.claimed.as_str()).unwrap_or(serde_json::from_str("{\"claimed\":[]}").unwrap());

            let mut claimstr = String::new();

            for i in claimed.claimed.iter_mut() {
                claimstr = claimstr + i.to_string().as_str();
            }

            //ui.heading(claimstr.as_str());

            //egui::ScrollArea::vertical().show(ui, |ui| {ui.heading(self.pumpkins.as_str());});

            let mut list: IndexMap<String, TileData> = serde_json::from_str(self.pumpkins.as_str()).unwrap();

            //ui.heading(list.1.unwrap_or("0".to_string()));

            let mut unclaimed = String::new();
            unclaimed.clear();

            //egui::Grid::new("grid").num_columns(2).show(ui, |ui| {

            ui.style_mut().spacing.window_margin = egui::Margin::symmetric(20, 10);

            egui::ScrollArea::vertical().show(ui, |ui| {

                let mut flag: bool = false;

            for (k, v) in list.iter_mut() {
                if !self.claimed.contains(k) {
                    flag = true;

                    let num = format!("{}:", k);
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.heading(RichText::new(num).color(Color32::ORANGE));
                        //unclaimed += format!(" {0} https://wplace.live/?lat={1}&lng={2} \n" , k, v.lat, v.lng).as_str();
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(8.0);
                            if ui.button(RichText::new("x").color(Color32::from_rgb(255, 50, 50))).clicked() {
                                self.claimed = update_claimed(self.claimed.clone(), k.clone());
                            }
                        if ui.button(RichText::new("open wplace")).clicked() {
                            let url = format!("https://wplace.live/?lat={0}&lng={1}", v.lat, v.lng);
                            ui.ctx().open_url(OpenUrl::new_tab(url))
                        }

                        })
                    });
                    //ui.end_row()
                }
            }


                    ui.add_space((ui.available_height() - 29.0).clamp(2.0,9999.9));
                    ui.add(egui::Separator::default().horizontal());
                    ui.add_space(2.0);
                    ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {

                        //ui.separator();

                        if ui.button(RichText::new("reset filter").color(Color32::from_rgb(255, 50, 50))).clicked() {
                            self.claimed.clear();
                        }
                    });



            });

            //});

            ui.heading(unclaimed.as_str());



        });
    }
}

fn fetch_list() -> String {
    let mut easy = Easy::new();
    let mut buf = Vec::new();
    easy.url("https://wplace.samuelscheit.com/tiles/pumpkin.json").unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        buf.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    std::str::from_utf8(buf.as_slice()).unwrap_or("{}").to_string()
}

fn update_claimed(mut claimed: String, input: String) -> String {
    if input.contains("{\"claimed\":[") {
        return input
    } else if !input.is_empty() && claimed.contains("]}") && claimed.contains("[") {
        claimed.pop();
        claimed.pop();
        let tempchar = claimed.pop().unwrap();
        let char = tempchar.clone();
        claimed.push(tempchar);
        if char == '[' {
            claimed += input.replace(" ", ",").as_str();
        } else {
            claimed.push(',');
            claimed += input.replace(" ", ",").as_str();
        }
        claimed.push(']');
        claimed.push('}');
    } else if !input.is_empty() {
        claimed = format!("{0}\"claimed\":[{1}]{2}", "{", input.replace(" ", ",").as_str(), "}" );
    }
    claimed
}

