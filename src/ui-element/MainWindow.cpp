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

    // do a null initialize on the settingsForm, this act as a sentinal
    // to avoid undefined behavior
    this->settingsForm = NULL;

    this->setup();
}

void MainWindow::setup() {
    connect(this->ui->actionExit, &QAction::triggered, this, &MainWindow::performExit);
    connect(this->ui->actionSettings, &QAction::triggered, this, &MainWindow::performSettings);
}


MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::performExit() {
    this->close();
}

void MainWindow::performSettings() {
    if(this->settingsForm == NULL) {
        this->settingsForm = new SettingsForm();
    }
    this->settingsForm->setWindowModality(Qt::WindowModality::ApplicationModal);
    this->settingsForm->show();
}

