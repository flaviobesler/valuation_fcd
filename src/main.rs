

use eframe::egui;
mod database;

#[tokio::main]
async fn main()-> eframe::Result{
    let options = eframe::NativeOptions::default();
    let database = database::Database::data();
    database.insert().await.expect("erro ao inserir no banco");

    Ok(())

    /*eframe::run_native(     "valuation",
                            options,
                            Box::new(|_cc| Ok(Box::new(MyApp::default()))),
                                )    */
                    
}






#[derive(Default)]
struct MyApp{
    valor: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame:&mut eframe::Frame ){
        egui::CentralPanel::default().show(ui, |ui|{
            ui.heading("teste valuation");
            ui.label("digite um numero");
            ui.add(egui::DragValue::new(&mut self.valor));

            if ui.button("click").clicked(){
                print!("clicou")};
            
            
        
        
        });
    
    }
    
} 
