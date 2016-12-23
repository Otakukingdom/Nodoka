//
// Created by mistlight on 12/22/2016.
//

#include "MainWindow.h"

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent), ui(new Ui::MainWindow()) {
    ui->setupUi( this );

}

MainWindow::~MainWindow() {
    delete ui;
}

