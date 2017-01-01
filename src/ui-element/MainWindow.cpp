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

    this->setup();
}

void MainWindow::setup() {
    connect(this->ui->actionExit, &QAction::triggered, this, &MainWindow::performExit);
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

