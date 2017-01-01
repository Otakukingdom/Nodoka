//
// Created by mistlight on 12/22/2016.
//

#include "MainWindow.h"
#include "SettingsForm.h"

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent), ui(new Ui::MainWindow()) {
    ui->setupUi( this );

    // a hack so the menu shows up on mac
    ui->menubar->setNativeMenuBar(false);

}

void MainWindow::setup() {

}


MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::performExit() {
    this->close();
}

void MainWindow::performSettings() {
    SettingsForm settingsForm;
    settingsForm.show();
}

