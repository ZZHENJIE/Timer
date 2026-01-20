pub mod mainwindow;

#[derive(Clone)]
pub enum WindowType {
    Main(mainwindow::MainWindow),
}
