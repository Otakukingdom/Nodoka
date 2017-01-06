//
// Created by mistlight on 12/22/2016.
//

#include "MainWindow.h"

MainWindow::MainWindow(Directory* directoryModel, Audiobook* audiobookModel, QWidget *parent) :
    QMainWindow(parent), ui(new Ui::MainWindow()) {
    ui->setupUi( this );

    // a hack so the menu shows up on mac
    ui->menubar->setNativeMenuBar(false);

    // set the model
    this->directoryModel = directoryModel;
    this->audiobookModel = audiobookModel;

    // initialize the settings form
    this->settingsForm = new SettingsForm(this->directoryModel);

    this->setup();
}

void MainWindow::setup() {
    connect(this->ui->actionExit, &QAction::triggered, this, &MainWindow::performExit);
    connect(this->ui->actionSettings, &QAction::triggered, this, &MainWindow::performSettings);

    this->ui->audiobookView->setModel(this->audiobookModel);
}


MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::performExit() {
    this->close();
}

void MainWindow::performSettings() {
    this->settingsForm->setWindowModality(Qt::WindowModality::ApplicationModal);
    this->settingsForm->show();
}

